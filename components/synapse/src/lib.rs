use libp2p::{
    gossipsub,
    ping,
    swarm::NetworkBehaviour,
    tcp,
    noise,
    yamux,
    PeerId,
    identity,
    SwarmBuilder,
};
use std::time::Duration;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use anyhow::Result;

// --- GOSSIPSUB CONFIGURATION ---
// We need to define how messages are identified (to prevent duplicates)
pub fn message_id_fn(message: &gossipsub::Message) -> gossipsub::MessageId {
    let mut s = DefaultHasher::new();
    message.data.hash(&mut s);
    gossipsub::MessageId::from(s.finish().to_string())
}
// -------------------------------

#[derive(NetworkBehaviour)]
pub struct SynapseBehavior {
    pub ping: ping::Behaviour,
    pub gossipsub: gossipsub::Behaviour, // NEW: The Gossip Layer
}

pub struct SynapseNode {
    pub peer_id: PeerId,
}

impl SynapseNode {
    pub async fn new() -> Result<Self> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        // 1. Configure GossipSub
        let gossip_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(1))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(message_id_fn) 
            .build()
            .map_err(|msg| anyhow::anyhow!("Gossip config failed: {}", msg))?;

        let gossip = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossip_config,
        ).map_err(|msg| anyhow::anyhow!("Gossip creation failed: {}", msg))?;

        // 2. Combine Behaviors
        let behavior = SynapseBehavior {
            ping: ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(1))),
            gossipsub: gossip,
        };

        // 3. Build Swarm
        let _swarm = SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|_| behavior)?
            .build();

        Ok(SynapseNode {
            peer_id: local_peer_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_gossip_initialization() {
        println!("TEST: Initializing GossipSub Node...");
        let node_result = SynapseNode::new().await;
        assert!(node_result.is_ok(), "GossipSub initialization failed");
    }
}
