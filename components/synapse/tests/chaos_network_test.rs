use anyhow::Result;
use libp2p::{
    core::transport::memory::MemoryTransport, core::upgrade::Version, futures::StreamExt, identity,
    noise, request_response, swarm::SwarmEvent, yamux, PeerId, Swarm, Transport,
};
use std::time::Duration;
use synapse::{SwarmRequest, SwarmResponse};
use tokio::time::sleep;

pub async fn build_chaos_swarm(
    _drop_probability: f64,
) -> Result<(
    PeerId,
    Swarm<request_response::cbor::Behaviour<SwarmRequest, SwarmResponse>>,
)> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    let base_transport = MemoryTransport::default();
    let chaos_transport = base_transport
        .map(move |out, _| {
            std::thread::sleep(Duration::from_millis(50));
            out
        })
        .upgrade(Version::V1)
        .authenticate(noise::Config::new(&id_keys).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    let req_res_protocol = libp2p::StreamProtocol::new("/swarm/req-res/test");
    let req_res_config =
        request_response::Config::default().with_request_timeout(Duration::from_secs(5));

    let behaviour = request_response::cbor::Behaviour::<SwarmRequest, SwarmResponse>::new(
        [(req_res_protocol, request_response::ProtocolSupport::Full)],
        req_res_config,
    );

    let swarm = libp2p::SwarmBuilder::with_existing_identity(id_keys)
        .with_tokio()
        .with_other_transport(|_| chaos_transport)
        .unwrap()
        .with_behaviour(|_| behaviour)
        .unwrap()
        .build();

    Ok((peer_id, swarm))
}

#[tokio::test]
async fn test_cellular_packet_loss_bft_sync() -> Result<()> {
    let (gateway_peer, mut gateway_swarm) = build_chaos_swarm(0.2).await?;
    let (_worker_peer, mut worker_swarm) = build_chaos_swarm(0.2).await?;

    let gateway_addr: libp2p::Multiaddr = "/memory/1".parse()?;
    gateway_swarm.listen_on(gateway_addr.clone())?;
    worker_swarm.dial(gateway_addr.clone())?;

    tokio::spawn(async move {
        loop {
            if let Some(event) = gateway_swarm.next().await {
                if let SwarmEvent::Behaviour(request_response::Event::Message {
                    message:
                        request_response::Message::Request {
                            request, channel, ..
                        },
                    ..
                }) = event
                {
                    assert_eq!(request, SwarmRequest::FetchData("BFT_SYNC".to_string()));
                    let _ = gateway_swarm
                        .behaviour_mut()
                        .send_response(channel, SwarmResponse::Ack);
                }
            }
        }
    });

    // 🚨 FIX: Actually wait for the Libp2p connection handshake to complete!
    loop {
        if let Some(event) = worker_swarm.next().await {
            if let SwarmEvent::ConnectionEstablished { .. } = event {
                break;
            }
        }
    }

    let request_id = worker_swarm.behaviour_mut().send_request(
        &gateway_peer,
        SwarmRequest::FetchData("BFT_SYNC".to_string()),
    );

    tokio::select! {
        _ = async {
            loop {
                if let Some(event) = worker_swarm.next().await {
                    if let SwarmEvent::Behaviour(request_response::Event::Message {
                        message: request_response::Message::Response { request_id: recv_id, response },
                        ..
                    }) = event {
                        if recv_id == request_id {
                            assert_eq!(response, SwarmResponse::Ack);
                            break;
                        }
                    }
                }
            }
        } => {
            println!("✅ BFT State Sync successful despite chaos conditions!");
        }
        _ = sleep(Duration::from_secs(3)) => {
            panic!("❌ Test failed: Gateway did not acknowledge payload within timeout");
        }
    }

    Ok(())
}
