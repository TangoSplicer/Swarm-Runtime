use libp2p::{
    gossipsub,
    mdns,
    ping,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp,
    noise,
    yamux,
    PeerId,
    identity,
    SwarmBuilder,
    Swarm,
    Multiaddr,
};
use std::time::{Duration, Instant};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use anyhow::Result;
use futures::StreamExt;

pub fn message_id_fn(message: &gossipsub::Message) -> gossipsub::MessageId {
    let mut s = DefaultHasher::new();
    message.data.hash(&mut s);
    gossipsub::MessageId::from(s.finish().to_string())
}

#[derive(NetworkBehaviour)]
pub struct SynapseBehavior {
    pub ping: ping::Behaviour,
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub struct SynapseNode {
    pub peer_id: PeerId,
    pub swarm: Swarm<SynapseBehavior>,
    pub topic: gossipsub::IdentTopic,
}

impl SynapseNode {
    pub async fn new() -> Result<Self> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
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

        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)?;

        let behavior = SynapseBehavior {
            ping: ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(1))),
            gossipsub: gossip,
            mdns,
        };

        let mut swarm = SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|_| behavior)?
            .build();

        let topic = gossipsub::IdentTopic::new("swarm-global");
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        Ok(SynapseNode {
            peer_id: local_peer_id,
            swarm,
            topic,
        })
    }

    pub async fn wait_for_event(&mut self) {
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                     println!("Synapse: Listening on {:?}", address);
                }
                SwarmEvent::Behaviour(SynapseBehaviorEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("Synapse: mDNS Discovered Peer: {:?}", peer_id);
                        self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                }
                SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer,
                    message_id: _,
                    message,
                })) => {
                    let text = String::from_utf8_lossy(&message.data);
                    println!("Synapse: RECEIVED MSG from {:?}: '{}'", peer, text);
                }
                _ => {}
            }
        }
    }

    pub fn publish(&mut self, message: String) -> Result<()> {
        self.swarm.behaviour_mut().gossipsub.publish(self.topic.clone(), message.as_bytes())?;
        Ok(())
    }

    pub fn dial_peer(&mut self, addr: String) -> Result<()> {
        let multiaddr: Multiaddr = addr.parse()?;
        self.swarm.dial(multiaddr)?;
        Ok(())
    }

    pub async fn wait_for_peers(&mut self) {
        println!("Synapse: Waiting for GossipSub mesh connection...");
        let start = Instant::now();
        
        loop {
            if self.swarm.network_info().num_peers() > 0 {
                println!("Synapse: Connection established!");
                break;
            }
            if start.elapsed() > Duration::from_secs(10) {
                println!("Synapse: Connection timed out.");
                break;
            }
            tokio::select! {
                _ = self.swarm.select_next_some() => {},
                _ = tokio::time::sleep(Duration::from_millis(100)) => {}
            }
        }
    }

    // NEW FUNCTION: Keeps the network alive for a set duration
    pub async fn drive_for(&mut self, duration: Duration) {
        let start = Instant::now();
        while start.elapsed() < duration {
            tokio::select! {
                _ = self.swarm.select_next_some() => {},
                _ = tokio::time::sleep(Duration::from_millis(10)) => {}
            }
        }
    }
}
