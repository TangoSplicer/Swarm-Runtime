use anyhow::{anyhow, Result};
use libp2p::{
    core::upgrade::Version, gossipsub, identify, kad, noise, request_response,
    swarm::NetworkBehaviour, tcp, yamux, PeerId, StreamProtocol, Swarm, Transport,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// The Unicast Request Payload
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwarmRequest {
    DispatchShard(String),
    SubmitResult(String),
    FetchData(String),
}

/// The Unicast Response Payload
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwarmResponse {
    Ack,
    Error(String),
    DataPayload(Vec<u8>),
}

#[derive(NetworkBehaviour)]
pub struct SynapseBehavior {
    pub gossipsub: gossipsub::Behaviour,
    // mdns has been intentionally removed to save WAN bandwidth on Android Edge Workers
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub identify: identify::Behaviour,
    pub req_res: request_response::cbor::Behaviour<SwarmRequest, SwarmResponse>,
}

pub struct SynapseNode {
    pub swarm: Swarm<SynapseBehavior>,
}

impl SynapseNode {
    // PHASE 9.1 & 15: Persistent Network Identity & Hardened Security
    pub async fn new(p2p_port: u16, seed: [u8; 32]) -> Result<Self> {
        // Convert our permanent 32-byte seed into a Libp2p Keypair
        let secret = libp2p::identity::ed25519::SecretKey::try_from_bytes(seed)?;
        let key = libp2p::identity::Keypair::from(libp2p::identity::ed25519::Keypair::from(secret));
        let peer_id = PeerId::from(key.public());

        // 1. TCP Transport with Noise & Yamux
        let transport = tcp::tokio::Transport::default()
            .upgrade(Version::V1)
            .authenticate(noise::Config::new(&key).unwrap())
            .multiplex(yamux::Config::default())
            .boxed();

        // 2. Hardened Gossipsub Config (Adaptive Backoff & Size Limits)
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(3)) // Relaxed from 1s to save battery
            .validation_mode(gossipsub::ValidationMode::Strict)
            .max_transmit_size(1024 * 1024) // 1MB Hard Limit to prevent network flooding
            .build()?;

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(key.clone()),
            gossipsub_config,
        )?;

        // 3. Kademlia DHT
        let store = kad::store::MemoryStore::new(peer_id);
        let kad_config = kad::Config::default();
        let kademlia = kad::Behaviour::with_config(peer_id, store, kad_config);

        // 4. Identify Protocol
        let identify_config = identify::Config::new("/swarm/1.0.0".to_string(), key.public());
        let identify = identify::Behaviour::new(identify_config);

        // 5. Hardened Request-Response (OOM Protection via Explicit Bounds)
        let req_res_protocol = StreamProtocol::new("/swarm/req-res/1.0.0");
        let req_res_config = request_response::Config::default()
            .with_max_request_size(2 * 1024 * 1024) // 2MB Bound
            .with_max_response_size(5 * 1024 * 1024) // 5MB Bound
            .with_request_timeout(Duration::from_secs(10));

        let req_res = request_response::cbor::Behaviour::<SwarmRequest, SwarmResponse>::new(
            [(req_res_protocol, request_response::ProtocolSupport::Full)],
            req_res_config,
        );

        // Assemble the behavior
        let behaviour = SynapseBehavior {
            gossipsub,
            kademlia,
            identify,
            req_res,
        };

        // Build the Swarm using Tokio
        let mut swarm =
            libp2p::SwarmBuilder::with_tokio_executor(transport, behaviour, peer_id).build();

        let listen_addr = format!("/ip4/0.0.0.0/tcp/{}", p2p_port);
        swarm.listen_on(listen_addr.parse()?)?;
        println!("👂 Listening on {}", listen_addr);

        Ok(Self { swarm })
    }

    pub fn subscribe(&mut self, topic: &str) -> Result<()> {
        let topic = gossipsub::IdentTopic::new(topic);
        self.swarm
            .behaviour_mut()
            .gossipsub
            .subscribe(&topic)
            .map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }

    pub fn publish_to_topic(&mut self, topic: &str, message: String) -> Result<()> {
        let topic = gossipsub::IdentTopic::new(topic);
        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, message.into_bytes())
            .map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }

    pub fn send_request(
        &mut self,
        peer: &PeerId,
        req: SwarmRequest,
    ) -> request_response::OutboundRequestId {
        self.swarm.behaviour_mut().req_res.send_request(peer, req)
    }
}
