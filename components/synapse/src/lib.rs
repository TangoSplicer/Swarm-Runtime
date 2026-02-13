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
use rand::Rng;

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

#[derive(Debug, Clone, PartialEq)]
pub enum RaftRole {
    Follower,
    Candidate,
    Leader,
}

pub struct SynapseNode {
    pub peer_id: PeerId,
    pub swarm: Swarm<SynapseBehavior>,
    pub role: RaftRole,
    pub current_term: u64,
    pub last_heartbeat: Instant,
    pub election_timeout: Duration,
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

        let mut rng = rand::thread_rng();
        // Election timeout between 7s and 12s
        let timeout_secs = rng.gen_range(7..12);

        Ok(SynapseNode {
            peer_id: local_peer_id,
            swarm,
            role: RaftRole::Follower,
            current_term: 0,
            last_heartbeat: Instant::now(),
            election_timeout: Duration::from_secs(timeout_secs),
        })
    }

    pub fn subscribe(&mut self, topic_name: &str) -> Result<()> {
        let topic = gossipsub::IdentTopic::new(topic_name);
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
        Ok(())
    }

    pub fn publish(&mut self, message: String) -> Result<()> {
        let topic = gossipsub::IdentTopic::new("swarm-global");
        self.swarm.behaviour_mut().gossipsub.publish(topic, message.as_bytes())?;
        Ok(())
    }

    pub fn publish_to_topic(&mut self, topic_name: &str, message: String) -> Result<()> {
        let topic = gossipsub::IdentTopic::new(topic_name);
        self.swarm.behaviour_mut().gossipsub.publish(topic, message.as_bytes())?;
        Ok(())
    }

    pub fn dial_peer(&mut self, addr: String) -> Result<()> {
        let multiaddr: Multiaddr = addr.parse()?;
        self.swarm.dial(multiaddr)?;
        Ok(())
    }

    pub async fn wait_for_peers(&mut self) {
        let start = Instant::now();
        loop {
            if self.swarm.network_info().num_peers() > 0 { break; }
            if start.elapsed() > Duration::from_secs(10) { break; }
            tokio::select! {
                _ = self.swarm.select_next_some() => {},
                _ = tokio::time::sleep(Duration::from_millis(100)) => {}
            }
        }
    }

    pub async fn wait_for_event(&mut self) {
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                     println!("Synapse: Listening on {:?}", address);
                }
                SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer,
                    message,
                    ..
                })) => {
                    let text = String::from_utf8_lossy(&message.data);
                    println!("Synapse: [Topic: {}] MSG from {:?}: '{}'", message.topic, peer, text);
                }
                _ => {}
            }
        }
    }

    pub async fn drive_for(&mut self, duration: Duration) {
        let start = Instant::now();
        while start.elapsed() < duration {
            tokio::select! {
                event = self.swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                             println!("Synapse: Listening on {:?}", address);
                        }
                        SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message {
                            propagation_source: peer,
                            message,
                            ..
                        })) => {
                            let text = String::from_utf8_lossy(&message.data);
                            println!("Synapse: [Topic: {}] MSG from {:?}: '{}'", message.topic, peer, text);
                        }
                        _ => {}
                    }
                },
                _ = tokio::time::sleep(Duration::from_millis(10)) => {}
            }
        }
    }

    pub fn check_election(&mut self) -> bool {
        if self.role == RaftRole::Follower && self.last_heartbeat.elapsed() > self.election_timeout {
            self.role = RaftRole::Leader;
            self.current_term += 1;
            println!("Raft: Timeout! I am assuming LEADERSHIP (Term {})", self.current_term);
            return true;
        }
        false
    }

    pub fn send_heartbeat(&mut self, shard_channel: &str) -> Result<()> {
        if self.role == RaftRole::Leader {
            let mut rng = rand::thread_rng();
            let nonce: u32 = rng.gen();
            let msg = format!("HEARTBEAT:{}:{}", self.current_term, nonce);
            self.publish_to_topic(shard_channel, msg)?;
        }
        Ok(())
    }
}
