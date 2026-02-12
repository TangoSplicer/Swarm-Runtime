use anyhow::Result; // FIX: Removed unused 'Context'
use clap::{Parser, Subcommand};
use std::time::Duration;
use libp2p::swarm::SwarmEvent;
use libp2p::gossipsub;
use futures::StreamExt; // FIX: Added StreamExt for select_next_some

use judge::Judge;
use synapse::{SynapseNode, SynapseBehaviorEvent};
use lazarus::{Lazarus, Monitorable};
use async_trait::async_trait;

#[derive(Parser)]
#[command(name = "swarm")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(long)]
        shard: Option<u64>,
    },
    Listen,
    Say {
        message: String,
        #[arg(long)]
        peer: Option<String>,
        #[arg(long)]
        shard: Option<u64>,
    },
    Check {
        path: String,
    },
}

struct ManagedNode {
    running: bool,
    role: String,
}

#[async_trait]
impl Monitorable for ManagedNode {
    fn name(&self) -> &str { &self.role }
    async fn is_alive(&self) -> bool { self.running }
    async fn restart(&mut self) -> Result<()> {
        self.running = true;
        Ok(())
    }
}

// --- DATABASE LOGIC ---
fn open_db(shard_id: u64) -> Result<sled::Db> {
    let path = format!("storage/shard_{}", shard_id);
    println!("System: Mounting Storage Partition -> {}", path);
    let db = sled::open(path)?;
    Ok(db)
}

fn process_command(db: &sled::Db, msg: &str) -> String {
    let parts: Vec<&str> = msg.splitn(3, ':').collect();
    match parts[0] {
        "SET" if parts.len() == 3 => {
            let key = parts[1];
            let val = parts[2];
            match db.insert(key, val.as_bytes()) {
                Ok(_) => {
                    let _ = db.flush();
                    format!("DB_WRITE_OK: {} = {}", key, val)
                },
                Err(e) => format!("DB_ERROR: {}", e),
            }
        },
        "GET" if parts.len() == 2 => {
            let key = parts[1];
            match db.get(key) {
                Ok(Some(val)) => {
                    let s = String::from_utf8_lossy(&val);
                    format!("DB_READ_OK: {} = {}", key, s)
                },
                Ok(None) => format!("DB_MISS: Key '{}' not found.", key),
                Err(e) => format!("DB_ERROR: {}", e),
            }
        },
        _ => format!("IGNORED: Unknown command format '{}'", msg),
    }
}
// ----------------------

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check { path } => {
            let compiler = prism::PrismCompiler::new();
            let code = std::fs::read_to_string(path).unwrap_or("fn main() {}".to_string());
            match compiler.transpile(&code, 4) {
                Ok(plan) => println!("SUCCESS:\n{}", plan),
                Err(e) => println!("FAILURE: {}", e),
            }
        }
        Commands::Listen => {
            println!("=== Swarm Network Listener ===");
            let mut p2p_node = SynapseNode::new().await?;
            println!("System: Connected! Peer ID: {}", p2p_node.peer_id);
            p2p_node.subscribe("swarm-shard-1")?;
            p2p_node.wait_for_event().await;
        }
        Commands::Say { message, peer, shard } => {
            println!("=== Swarm Broadcaster ===");
            let mut p2p_node = SynapseNode::new().await?;
            if let Some(addr) = peer {
                println!("System: Dialing peer at {}...", addr);
                p2p_node.dial_peer(addr.clone())?;
            }
            p2p_node.wait_for_peers().await;
            p2p_node.drive_for(Duration::from_secs(3)).await;

            if let Some(shard_id) = shard {
                let topic = format!("swarm-shard-{}", shard_id);
                println!("System: Sending to SHARD {}: '{}'", shard_id, message);
                p2p_node.publish_to_topic(&topic, message.clone())?;
            } else {
                p2p_node.publish(message.clone())?;
            }
            
            println!("SUCCESS: Command Sent.");
            p2p_node.drive_for(Duration::from_secs(1)).await;
        }
        Commands::Start { shard } => {
            println!("=== Swarm Runtime v0.4.0 ===");
            println!("System: Boot sequence initiated...");

            let _judge = Judge::new()?;
            let mut p2p_node = SynapseNode::new().await?;
            
            let (role_name, db) = match shard {
                Some(id) => {
                    let channel = format!("swarm-shard-{}", id);
                    println!("System: ROLE ASSIGNED -> Worker for Shard {}", id);
                    p2p_node.subscribe(&channel)?;
                    
                    // FIX: Added dereference (*id) here
                    let db = open_db(*id)?;
                    (format!("Shard {} Worker", id), Some(db))
                },
                None => {
                    println!("System: ROLE ASSIGNED -> General Observer");
                    ("Observer".to_string(), None)
                }
            };

            println!("System: Node is live! Peer ID: {}", p2p_node.peer_id);
            
            let mut managed_node = ManagedNode { running: true, role: role_name };
            tokio::spawn(async move {
                let watchdog = Lazarus::new(5);
                let _ = watchdog.watch(&mut managed_node).await;
            });

            println!("=== System Fully Operational ===");
            
            loop {
                tokio::select! {
                    event = p2p_node.swarm.select_next_some() => {
                        match event {
                            SwarmEvent::NewListenAddr { address, .. } => {
                                 println!("Synapse: Listening on {:?}", address);
                            },
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message {
                                propagation_source: _peer,
                                message_id: _,
                                message,
                            })) => {
                                let text = String::from_utf8_lossy(&message.data);
                                println!("Synapse: [Topic: {}] CMD: '{}'", message.topic, text);

                                if let Some(database) = &db {
                                    let result = process_command(database, &text);
                                    println!("Storage: {}", result);
                                }
                            },
                            _ => {}
                        }
                    },
                    _ = tokio::time::sleep(Duration::from_millis(10)) => {}
                }
            }
        }
    }
    Ok(())
}
