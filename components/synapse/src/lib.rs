use libp2p::{
    gossipsub, mdns, noise, kad, identify, swarm::NetworkBehaviour, tcp, yamux, PeerId, Swarm,
};
use std::time::Duration;
use anyhow::{Result, anyhow};

#[derive(NetworkBehaviour)]
pub struct SynapseBehavior {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub identify: identify::Behaviour, // NEW: The "ID Card" protocol
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
                // 1. GossipSub
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(1))
                    .validation_mode(gossipsub::ValidationMode::Permissive)
                    .build()
                    .map_err(|e| anyhow!(e))?;

                // 2. Kademlia
                let store = kad::store::MemoryStore::new(peer_id);
                let kad_config = kad::Config::default();

                // 3. mDNS
                let mdns_config = mdns::Config::default();

                // 4. Identify (Fixes connection drops)
                let identify_config = identify::Config::new(
                    "swarm/1.0.0".to_string(),
                    key.public(),
                );

                Ok(SynapseBehavior {
                    gossipsub: gossipsub::Behaviour::new(gossipsub::MessageAuthenticity::Signed(key.clone()), gossipsub_config)?,
                    mdns: mdns::tokio::Behaviour::new(mdns_config, peer_id)?,
                    kademlia: kad::Behaviour::with_config(peer_id, store, kad_config),
                    identify: identify::Behaviour::new(identify_config),
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
        self.swarm.behaviour_mut().gossipsub.publish(topic, message.as_bytes()).map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }
}
