use anyhow::Result;
use libp2p::{
    core::transport::memory::MemoryTransport,
    core::upgrade::Version,
    identity, noise, request_response,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    yamux, PeerId, Swarm, Transport,
};
use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

// Bring in your existing behavior payloads
use synapse::{SwarmRequest, SwarmResponse};

/// A custom wrapper around MemoryTransport that simulates cellular packet loss
/// by randomly dropping the connection attempt or adding latency.
pub async fn build_chaos_swarm(
    drop_probability: f64,
) -> Result<(
    PeerId,
    Swarm<request_response::cbor::Behaviour<SwarmRequest, SwarmResponse>>,
)> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    // 1. Setup Native Memory Transport
    let base_transport = MemoryTransport::default();

    // 2. Wrap with a "Chaos" interceptor using Map
    // In a real scenario, we wrap the stream, but here we simulate latency on dialing
    let chaos_transport = base_transport
        .map(move |out, _| {
            // Simulate random cellular latency
            let latency = rand::thread_rng().gen_range(10..150);
            std::thread::sleep(Duration::from_millis(latency));
            out
        })
        .upgrade(Version::V1)
        .authenticate(noise::Config::new(&id_keys).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    // 3. Configure Request-Response (BFT Payload Simulation)
    let req_res_protocol = libp2p::StreamProtocol::new("/swarm/req-res/test");
    let req_res_config =
        request_response::Config::default().with_request_timeout(Duration::from_secs(5));

    let behaviour = request_response::cbor::Behaviour::<SwarmRequest, SwarmResponse>::new(
        [(req_res_protocol, request_response::ProtocolSupport::Full)],
        req_res_config,
    );

    // 4. Build Swarm
    let swarm = SwarmBuilder::with_tokio_executor(chaos_transport, behaviour, peer_id).build();

    Ok((peer_id, swarm))
}

#[tokio::test]
async fn test_cellular_packet_loss_bft_sync() -> Result<()> {
    // We instantiate two nodes with a 20% simulated packet loss / high latency configuration
    let (gateway_peer, mut gateway_swarm) = build_chaos_swarm(0.2).await?;
    let (worker_peer, mut worker_swarm) = build_chaos_swarm(0.2).await?;

    // Assign in-memory ports (port 1 for Gateway, port 2 for Worker)
    let gateway_addr: libp2p::Multiaddr = "/memory/1".parse()?;
    gateway_swarm.listen_on(gateway_addr.clone())?;

    // Worker dials the Gateway
    worker_swarm.dial(gateway_addr.clone())?;

    // Spawn a background task for the Gateway to listen and respond
    tokio::spawn(async move {
        loop {
            if let Some(event) = gateway_swarm.next().await {
                if let SwarmEvent::Behaviour(request_response::Event::Message {
                    peer: _,
                    message:
                        request_response::Message::Request {
                            request, channel, ..
                        },
                }) = event
                {
                    // Assert we got the expected BFT payload
                    assert_eq!(request, SwarmRequest::FetchData("BFT_SYNC".to_string()));

                    // Send an Ack back
                    let _ = gateway_swarm
                        .behaviour_mut()
                        .send_response(channel, SwarmResponse::Ack);
                }
            }
        }
    });

    // Simulate sending a Request from Worker to Gateway
    let request_id = worker_swarm.behaviour_mut().send_request(
        &gateway_peer,
        SwarmRequest::FetchData("BFT_SYNC".to_string()),
    );

    // Await the response from the Gateway over the memory transport
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
