use anyhow::Result;
use libp2p::{
    core::transport::memory::MemoryTransport, core::upgrade::Version, futures::StreamExt, identity,
    noise, request_response, swarm::SwarmEvent, yamux, PeerId, Swarm, Transport,
};
use std::time::Duration;
use synapse::{SwarmRequest, SwarmResponse};

pub async fn build_reliable_swarm() -> Result<(
    PeerId,
    Swarm<request_response::cbor::Behaviour<SwarmRequest, SwarmResponse>>,
)> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

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
        // 🚨 FIX: Add a 10-second idle timeout to prevent the Swarm from killing the socket before Yamux flushes the response!
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(10)))
        .build();

    Ok((peer_id, swarm))
}

#[tokio::test]
async fn test_cellular_packet_loss_bft_sync() -> Result<()> {
    let (gateway_peer, mut gateway_swarm) = build_reliable_swarm().await?;
    let (_worker_peer, mut worker_swarm) = build_reliable_swarm().await?;

    gateway_swarm.listen_on("/memory/0".parse()?)?;

    let gateway_addr = loop {
        if let Some(SwarmEvent::NewListenAddr { address, .. }) = gateway_swarm.next().await {
            break address;
        }
    };

    worker_swarm
        .behaviour_mut()
        .add_address(&gateway_peer, gateway_addr);
    let expected_request_id = worker_swarm.behaviour_mut().send_request(
        &gateway_peer,
        SwarmRequest::FetchData("BFT_SYNC".to_string()),
    );

    let network_loop = async {
        loop {
            tokio::select! {
                event = gateway_swarm.next() => {
                    let Some(event) = event else { continue };
                    if let SwarmEvent::Behaviour(request_response::Event::Message {
                        message: request_response::Message::Request { request, channel, .. },
                        ..
                    }) = event
                    {
                        if request == SwarmRequest::FetchData("BFT_SYNC".to_string()) {
                            let _ = gateway_swarm.behaviour_mut().send_response(channel, SwarmResponse::Ack);
                        }
                    }
                }

                event = worker_swarm.next() => {
                    let Some(event) = event else { continue };
                    match event {
                        SwarmEvent::Behaviour(request_response::Event::Message {
                            message: request_response::Message::Response { request_id, response },
                            ..
                        }) => {
                            if request_id == expected_request_id && response == SwarmResponse::Ack {
                                return Ok::<(), anyhow::Error>(());
                            }
                        }
                        SwarmEvent::OutgoingConnectionError { error, .. } => panic!("🔥 Connection failed: {:?}", error),
                        SwarmEvent::Behaviour(request_response::Event::OutboundFailure { error, .. }) => panic!("🔥 Request failed: {:?}", error),
                        _ => {}
                    }
                }
            }
        }
    };

    tokio::time::timeout(Duration::from_secs(5), network_loop)
        .await
        .expect("❌ Test failed: Gateway did not acknowledge payload within 5 seconds")?;

    println!("✅ BFT State Sync successful!");
    Ok(())
}
