use libp2p::{
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
use anyhow::Result;

#[derive(NetworkBehaviour)]
pub struct SynapseBehavior {
    pub ping: ping::Behaviour,
}

pub struct SynapseNode {
    pub peer_id: PeerId,
}

impl SynapseNode {
    pub async fn new() -> Result<Self> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        println!("Synapse: Local Peer ID generated: {:?}", local_peer_id);

        let behavior = SynapseBehavior {
            ping: ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(1))),
        };

        // Simplified builder that handles transport internally
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
