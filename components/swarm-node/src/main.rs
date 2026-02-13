use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::{Duration, Instant};
use libp2p::swarm::SwarmEvent;
use libp2p::gossipsub;
use futures::StreamExt;
use axum::{extract::{State, Json}, routing::post, Router};
use std::sync::Arc;
use dashmap::DashMap;
use uuid::Uuid;
use tokio::sync::oneshot;
use serde::Deserialize;
use base64::{engine::general_purpose, Engine as _};

use synapse::{SynapseNode, SynapseBehaviorEvent, RaftRole};
use judge::Judge;

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
    Say { message: String, #[arg(long)] peer: Option<String> },
}

#[derive(Deserialize)]
struct DeployRequest {
    wasm_base64: String,
}

struct AppState {
    pub node_tx: tokio::sync::mpsc::UnboundedSender<String>,
    pub pending_tasks: Arc<DashMap<String, oneshot::Sender<String>>>,
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
        Commands::Say { message, peer } => {
            let mut p2p_node = SynapseNode::new().await?;
            if let Some(addr) = peer { let _ = p2p_node.dial_peer(addr.to_string()); }
            p2p_node.wait_for_peers().await;
            p2p_node.drive_for(Duration::from_secs(1)).await;
            p2p_node.publish(message.to_string())?;
        }

        Commands::Gateway { port } => {
            println!("=== Swarm HTTP Gateway v0.6.0 ===");
            let mut p2p_node = SynapseNode::new().await?;
            p2p_node.subscribe("swarm-shard-1")?;
            
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            let pending = Arc::new(DashMap::new());
            let shared_state = Arc::new(AppState { node_tx: tx, pending_tasks: pending.clone() });

            let app = Router::new()
                .route("/deploy", post(|State(state): State<Arc<AppState>>, Json(payload): Json<DeployRequest>| async move {
                    let task_id = Uuid::new_v4().to_string();
                    let (otx, orx) = oneshot::channel();
                    state.pending_tasks.insert(task_id.clone(), otx);

                    let _ = state.node_tx.send(format!("TASK:{}:{}", task_id, payload.wasm_base64));
                    
                    match tokio::time::timeout(Duration::from_secs(10), orx).await {
                        Ok(Ok(res)) => format!("EXECUTION_SUCCESS: Result = {}\n", res),
                        _ => {
                            state.pending_tasks.remove(&task_id);
                            format!("EXECUTION_TIMEOUT: Task {} failed\n", task_id)
                        }
                    }
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
                                    SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { ref message, .. })) => {
                                        let text = String::from_utf8_lossy(&message.data);
                                        if text.starts_with("TASK_RESULT:") {
                                            let parts: Vec<&str> = text.split(':').collect();
                                            if parts.len() == 3 {
                                                if let Some((_, otx)) = pending.remove(parts[1]) {
                                                    let _ = otx.send(parts[2].to_string());
                                                }
                                            }
                                        }
                                    },
                                    SwarmEvent::Behaviour(SynapseBehaviorEvent::Mdns(libp2p::mdns::Event::Discovered(list))) => {
                                        for (_, addr) in list { let _ = p2p_node.dial_peer(addr.to_string()); }
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
            println!("=== Swarm Worker v0.6.0 ===");
            let mut p2p_node = SynapseNode::new().await?;
            let mut judge = Judge::new()?;
            let shard_id = shard.unwrap_or(1);
            let shard_channel = format!("swarm-shard-{}", shard_id);
            p2p_node.subscribe(&shard_channel)?;
            println!("=== Worker Live (Shard {}) ===", shard_id);

            loop {
                tokio::select! {
                    event = p2p_node.swarm.select_next_some() => {
                        match event {
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Mdns(libp2p::mdns::Event::Discovered(list))) => {
                                for (_, addr) in list { let _ = p2p_node.dial_peer(addr.to_string()); }
                            },
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { ref message, .. })) => {
                                let text = String::from_utf8_lossy(&message.data);
                                let parts: Vec<&str> = text.splitn(3, ':').collect();
                                
                                if text.starts_with("TASK:") && parts.len() == 3 {
                                    let task_id = parts[1];
                                    let b64 = parts[2];
                                    if let Ok(wasm_bytes) = general_purpose::STANDARD.decode(b64) {
                                        let result = match judge.execute(&wasm_bytes) {
                                            Ok(res) => res.to_string(),
                                            Err(e) => format!("Error: {}", e),
                                        };
                                        let _ = p2p_node.publish_to_topic(&shard_channel, format!("TASK_RESULT:{}:{}", task_id, result));
                                    }
                                } else if text.starts_with("HEARTBEAT:") {
                                    p2p_node.last_heartbeat = Instant::now();
                                }
                            },
                            _ => {}
                        }
                    },
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        p2p_node.check_election();
                        if p2p_node.role == RaftRole::Leader { let _ = p2p_node.send_heartbeat(&shard_channel); }
                    }
                }
            }
        }
    }
    Ok(())
}
