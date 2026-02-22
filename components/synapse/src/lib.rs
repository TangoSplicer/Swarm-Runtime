use libp2p::{
    gossipsub, mdns, noise, kad, identify, request_response, swarm::NetworkBehaviour, tcp, yamux, PeerId, Swarm, StreamProtocol
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use anyhow::{Result, anyhow};

/// The Unicast Request Payload
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwarmRequest {
    /// Gateway -> Worker: Sending the Shard payload (JSON String)
    DispatchShard(String),
    /// Worker -> Gateway: Returning the calculated Result (JSON String)
    SubmitResult(String),
}

/// The Unicast Response Payload
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwarmResponse {
    /// Immediate Decoupled ACK to close the stream
    Ack,
    /// Rejection or Error
    Error(String),
}

#[derive(NetworkBehaviour)]
pub struct SynapseBehavior {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub identify: identify::Behaviour,
    pub req_res: request_response::cbor::Behaviour<SwarmRequest, SwarmResponse>, // The New Data Plane
}

pub struct SynapseNode {
    pub swarm: Swarm<SynapseBehavior>,
}

impl SynapseNode {
    pub async fn new(p2p_port: u16) -> Result<Self> {
        let id_keys = libp2p::identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());

        println!("🆔 Local Peer ID: {}", peer_id);

        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(id_keys)
            .with_tokio()
            .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)?
            .with_behaviour(|key| {
                // 1. GossipSub (Control Plane)
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(1))
                    .validation_mode(gossipsub::ValidationMode::Permissive)
                    .build()
                    .map_err(|e| anyhow!("{:?}", e))?;

                // 2. Kademlia
                let mut kad_config = kad::Config::default();
                kad_config.set_protocol_names(vec![StreamProtocol::new("/swarm/kad/1.0.0")]);
                let store = kad::store::MemoryStore::new(peer_id);

                // 3. mDNS
                let mdns_config = mdns::Config::default();

                // 4. Identify
                let identify_config = identify::Config::new(
                    "swarm/1.0.0".to_string(),
                    key.public(),
                );

                // 5. Request-Response (Data Plane)
                let req_res_protocol = StreamProtocol::new("/swarm/req-res/1.0.0");
                let req_res_config = request_response::Config::default();
                let req_res = request_response::cbor::Behaviour::<SwarmRequest, SwarmResponse>::new(
                    [(req_res_protocol, request_response::ProtocolSupport::Full)],
                    req_res_config,
                );

                Ok(SynapseBehavior {
                    gossipsub: gossipsub::Behaviour::new(gossipsub::MessageAuthenticity::Signed(key.clone()), gossipsub_config)?,
                    mdns: mdns::tokio::Behaviour::new(mdns_config, peer_id)?,
                    kademlia: kad::Behaviour::with_config(peer_id, store, kad_config),
                    identify: identify::Behaviour::new(identify_config),
                    req_res,
                })
            })?
            .build();

        let listen_addr = format!("/ip4/0.0.0.0/tcp/{}", p2p_port);
        swarm.listen_on(listen_addr.parse()?)?;
        println!("👂 Listening on {}", listen_addr);

        Ok(Self { swarm })
    }

    pub fn subscribe(&mut self, topic: &str) -> Result<()> {
        let topic = gossipsub::IdentTopic::new(topic);
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic).map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }

    pub fn publish_to_topic(&mut self, topic: &str, message: String) -> Result<()> {
        let topic = gossipsub::IdentTopic::new(topic);
        self.swarm.behaviour_mut().gossipsub.publish(topic, message.into_bytes()).map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }

    /// Dispatch a 1-to-1 Unicast Request
    pub fn send_request(&mut self, peer: &PeerId, req: SwarmRequest) -> request_response::OutboundRequestId {
        self.swarm.behaviour_mut().req_res.send_request(peer, req)
    }

    /// Respond to an incoming Unicast Request
    pub fn send_response(&mut self, ch: request_response::ResponseChannel<SwarmResponse>, res: SwarmResponse) {
        let _ = self.swarm.behaviour_mut().req_res.send_response(ch, res);
    }
}
