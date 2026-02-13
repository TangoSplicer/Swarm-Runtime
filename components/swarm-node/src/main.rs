use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::{Duration, Instant};
use libp2p::swarm::SwarmEvent;
use libp2p::gossipsub;
use futures::StreamExt;
use axum::{extract::{Query, State}, routing::get, Router};
use std::sync::Arc;
use dashmap::DashMap;
use uuid::Uuid;
use tokio::sync::oneshot;

use synapse::{SynapseNode, SynapseBehaviorEvent, RaftRole};

#[derive(Parser)]
#[command(name = "swarm")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start { #[arg(long)] shard: Option<u64> },
    Gateway { #[arg(long, default_value = "8080")] port: u16 },
    Say { 
        message: String, 
        #[arg(long)] peer: Option<String>, 
        #[arg(long)] shard: Option<u64> 
    },
}

struct AppState {
    pub node_tx: tokio::sync::mpsc::UnboundedSender<String>,
    pub pending_responses: Arc<DashMap<String, oneshot::Sender<String>>>,
}

fn open_db(shard_id: u64) -> Result<sled::Db> {
    let path = format!("storage/shard_{}", shard_id);
    let db = sled::open(path)?;
    Ok(db)
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Say { message, peer, shard: _ } => {
            let mut p2p_node = SynapseNode::new().await?;
            if let Some(addr) = peer { let _ = p2p_node.dial_peer(addr.to_string()); }
            p2p_node.wait_for_peers().await;
            p2p_node.drive_for(Duration::from_secs(1)).await;
            p2p_node.publish(message.to_string())?;
        }

        Commands::Gateway { port } => {
            println!("=== Swarm HTTP Gateway v0.4.0 ===");
            let mut p2p_node = SynapseNode::new().await?;
            p2p_node.subscribe("swarm-shard-1")?;
            
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            let pending: Arc<DashMap<String, oneshot::Sender<String>>> = Arc::new(DashMap::new());
            let shared_state = Arc::new(AppState { 
                node_tx: tx, 
                pending_responses: pending.clone() 
            });

            let app = Router::new()
                .route("/set", get(|Query(params): Query<std::collections::HashMap<String, String>>, State(state): State<Arc<AppState>>| async move {
                    if let (Some(k), Some(v)) = (params.get("key"), params.get("val")) {
                        let _ = state.node_tx.send(format!("SET:{}:{}", k, v));
                        return format!("OK: Set request broadcast\n");
                    }
                    "Error: key/val required\n".to_string()
                }))
                .route("/get", get(|Query(params): Query<std::collections::HashMap<String, String>>, State(state): State<Arc<AppState>>| async move {
                    if let Some(k) = params.get("key") {
                        let req_id = Uuid::new_v4().to_string();
                        let (tx, rx) = oneshot::channel();
                        state.pending_responses.insert(req_id.clone(), tx);
                        let _ = state.node_tx.send(format!("GET:{}:{}", k, req_id));
                        
                        // Slightly longer timeout for cold starts
                        match tokio::time::timeout(Duration::from_secs(6), rx).await {
                            Ok(Ok(val)) => return format!("VALUE: {}\n", val),
                            _ => {
                                state.pending_responses.remove(&req_id);
                                return "Error: Timeout (Mesh still warm-up or Key not found)\n".to_string();
                            }
                        }
                    }
                    "Error: key required\n".to_string()
                }))
                .with_state(shared_state);

            let addr = std::net::SocketAddr::from(([0, 0, 0, 0], *port));
            println!("Gateway: Listening on http://{}", addr);

            tokio::select! {
                _ = axum::Server::bind(&addr).serve(app.into_make_service()) => {},
                _ = async {
                    loop {
                        tokio::select! {
                            Some(cmd) = rx.recv() => {
                                let _ = p2p_node.publish_to_topic("swarm-shard-1", cmd);
                            },
                            event = p2p_node.swarm.select_next_some() => {
                                match event {
                                    SwarmEvent::Behaviour(SynapseBehaviorEvent::Mdns(libp2p::mdns::Event::Discovered(list))) => {
                                        for (_, addr) in list { let _ = p2p_node.dial_peer(addr.to_string()); }
                                    },
                                    SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                        let text = String::from_utf8_lossy(&message.data);
                                        if text.starts_with("RESULT:") {
                                            let parts: Vec<&str> = text.split(':').collect();
                                            if parts.len() == 3 {
                                                if let Some((_, tx)) = pending.remove(parts[1]) {
                                                    let _ = tx.send(parts[2].to_string());
                                                }
                                            }
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                } => {}
            }
        }

        Commands::Start { shard } => {
            println!("=== Swarm Worker v0.4.0 ===");
            let mut p2p_node = SynapseNode::new().await?;
            let shard_id = shard.unwrap_or(1);
            let shard_channel = format!("swarm-shard-{}", shard_id);
            p2p_node.subscribe(&shard_channel)?;
            let db = open_db(shard_id)?;
            println!("=== Worker Live (Shard {}) ===", shard_id);
            let mut heartbeat_interval = tokio::time::interval(Duration::from_secs(1));

            loop {
                tokio::select! {
                    event = p2p_node.swarm.select_next_some() => {
                        match event {
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Mdns(libp2p::mdns::Event::Discovered(list))) => {
                                for (_, addr) in list { let _ = p2p_node.dial_peer(addr.to_string()); }
                            },
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                let text = String::from_utf8_lossy(&message.data);
                                let parts: Vec<&str> = text.split(':').collect();
                                if text.starts_with("SET:") && p2p_node.role == RaftRole::Leader && parts.len() == 3 {
                                    let _ = p2p_node.publish_to_topic(&shard_channel, text.replace("SET:", "REPLICATE:"));
                                    let _ = db.insert(parts[1], parts[2].as_bytes());
                                    let _ = db.flush();
                                    println!("Storage: Saved {}={}", parts[1], parts[2]);
                                } else if text.starts_with("REPLICATE:") && parts.len() == 3 {
                                    let _ = db.insert(parts[1], parts[2].as_bytes());
                                    let _ = db.flush();
                                    println!("Storage: Replicated {}={}", parts[1], parts[2]);
                                } else if text.starts_with("GET:") && parts.len() == 3 {
                                    let key = parts[1];
                                    let req_id = parts[2];
                                    let val = match db.get(key) {
                                        Ok(Some(v)) => String::from_utf8_lossy(&v).to_string(),
                                        _ => "NOT_FOUND".to_string(),
                                    };
                                    let _ = p2p_node.publish_to_topic(&shard_channel, format!("RESULT:{}:{}", req_id, val));
                                } else if text.starts_with("HEARTBEAT:") {
                                    p2p_node.last_heartbeat = Instant::now();
                                }
                            },
                            _ => {}
                        }
                    },
                    _ = heartbeat_interval.tick() => {
                        if p2p_node.role == RaftRole::Leader { let _ = p2p_node.send_heartbeat(&shard_channel); }
                        p2p_node.check_election();
                    }
                }
            }
        }
    }
    Ok(())
}
