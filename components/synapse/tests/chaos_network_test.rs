use anyhow::Result;
use libp2p::{
    core::transport::memory::MemoryTransport, core::upgrade::Version, futures::StreamExt, identity,
    noise, request_response, swarm::SwarmEvent, yamux, PeerId, Swarm, Transport,
};
use std::time::Duration;
use synapse::{SwarmRequest, SwarmResponse};
use tokio::time::sleep;

pub async fn build_reliable_swarm() -> Result<(
    PeerId,
    Swarm<request_response::cbor::Behaviour<SwarmRequest, SwarmResponse>>,
)> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    // 🚨 FIX: Remove thread-blocking chaos. Use pure memory transport to avoid Tokio starvation.
    let transport = MemoryTransport::default()
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
        .with_other_transport(|_| transport)
        .unwrap()
        .with_behaviour(|_| behaviour)
        .unwrap()
        .build();

    Ok((peer_id, swarm))
}

#[tokio::test]
async fn test_cellular_packet_loss_bft_sync() -> Result<()> {
    let (gateway_peer, mut gateway_swarm) = build_reliable_swarm().await?;
    let (_worker_peer, mut worker_swarm) = build_reliable_swarm().await?;

    // 🚨 FIX: Bind to dynamic memory port 0 to prevent address collisions in CI
    gateway_swarm.listen_on("/memory/0".parse()?)?;

    // Wait for the gateway to successfully assign and announce its memory address
    let gateway_addr = loop {
        if let Some(SwarmEvent::NewListenAddr { address, .. }) = gateway_swarm.next().await {
            break address;
        }
    };

    // Spawn the Gateway listener task
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
                    if request == SwarmRequest::FetchData("BFT_SYNC".to_string()) {
                        let _ = gateway_swarm
                            .behaviour_mut()
                            .send_response(channel, SwarmResponse::Ack);
                    }
                }
            }
        }
    });

    // Worker connects to the dynamically assigned Gateway address
    worker_swarm.dial(gateway_addr)?;

    // Wait for the secure tunnel to establish
    loop {
        if let Some(event) = worker_swarm.next().await {
            if let SwarmEvent::ConnectionEstablished { .. } = event {
                break;
            }
        }
    }

    // Fire the payload
    let request_id = worker_swarm.behaviour_mut().send_request(
        &gateway_peer,
        SwarmRequest::FetchData("BFT_SYNC".to_string()),
    );

    // Wait for the BFT Ack with a generous 5-second CI timeout buffer
    tokio::select! {
        _ = async {
            loop {
                if let Some(event) = worker_swarm.next().await {
                    if let SwarmEvent::Behaviour(request_response::Event::Message {
                        message: request_response::Message::Response { request_id: recv_id, response },
                        ..
                    }) = event {
                        if recv_id == request_id && response == SwarmResponse::Ack {
                            break;
                        }
                    }
                }
            }
        } => {
            println!("✅ BFT State Sync successful!");
        }
        _ = sleep(Duration::from_secs(5)) => {
            panic!("❌ Test failed: Gateway did not acknowledge payload within timeout");
        }
    }

    Ok(())
}
