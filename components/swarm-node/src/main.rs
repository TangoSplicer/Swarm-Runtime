use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::{Duration, Instant};
use libp2p::swarm::SwarmEvent;
use libp2p::gossipsub;
use futures::StreamExt;
use axum::{extract::{State, Json}, routing::post, routing::get, Router, response::Html};
use std::sync::Arc;
use dashmap::DashMap;
use uuid::Uuid;
use tokio::sync::{oneshot, Mutex};
use serde::{Deserialize, Serialize};
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

// Added Clone here to fix the move error
#[derive(Serialize, Clone)]
struct SwarmStatus {
    version: String,
    role: String,
    peers_count: usize,
}

struct AppState {
    pub node_tx: tokio::sync::mpsc::UnboundedSender<String>,
    pub pending_tasks: Arc<DashMap<String, oneshot::Sender<String>>>,
    pub stats: Arc<Mutex<SwarmStatus>>,
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
            println!("=== Swarm HTTP Gateway v0.6.2 ===");
            let mut p2p_node = SynapseNode::new().await?;
            p2p_node.subscribe("swarm-shard-1")?;
            
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            let pending = Arc::new(DashMap::new());
            let stats = Arc::new(Mutex::new(SwarmStatus {
                version: "0.6.1".to_string(),
                role: "Gateway".to_string(),
                peers_count: 0,
            }));

            let shared_state = Arc::new(AppState { 
                node_tx: tx, 
                pending_tasks: pending.clone(),
                stats: stats.clone(),
            });

            let pending_clone = pending.clone();
            let stats_clone = stats.clone();
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        Some(cmd) = rx.recv() => {
                            let _ = p2p_node.publish_to_topic("swarm-shard-1", cmd);
                        },
                        event = p2p_node.swarm.select_next_some() => {
                            match event {
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                    let text = String::from_utf8_lossy(&message.data);
                                    if text.starts_with("TASK_RESULT:") {
                                        let parts: Vec<&str> = text.split(':').collect();
                                        if parts.len() == 3 {
                                            if let Some((_, otx)) = pending_clone.remove(parts[1]) {
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
                            let mut s = stats_clone.lock().await;
                            s.peers_count = p2p_node.swarm.connected_peers().count();
                        }
                    }
                }
            });

            let app = Router::new()
                .route("/", get(|| async { Html(include_str!("dashboard.html")) }))
                .route("/status", get(|State(state): State<Arc<AppState>>| async move {
                    let s = state.stats.lock().await;
                    // Fix: Return a clone of the status to avoid moving out of the MutexGuard
                    Json(s.clone())
                }))
                .route("/deploy", post(|State(state): State<Arc<AppState>>, Json(payload): Json<DeployRequest>| async move {
                    let task_id = Uuid::new_v4().to_string();
                    let (otx, orx) = oneshot::channel();
                    state.pending_tasks.insert(task_id.clone(), otx);
                    let _ = state.node_tx.send(format!("TASK:{}:{}", task_id, payload.wasm_base64));
                    match tokio::time::timeout(Duration::from_secs(10), orx).await {
                        Ok(Ok(res)) => format!("EXECUTION_SUCCESS: Result = {}\n", res),
                        _ => format!("EXECUTION_TIMEOUT\n"),
                    }
                }))
                .with_state(shared_state);

            let addr = std::net::SocketAddr::from(([0, 0, 0, 0], *port));
            println!("Gateway: Listening on http://{}", addr);
            axum::Server::bind(&addr).serve(app.into_make_service()).await?;
        }

        Commands::Start { shard } => {
            println!("=== Swarm Worker v0.6.2 ===");
            let mut p2p_node = SynapseNode::new().await?;
            let shard_id = shard.unwrap_or(1);
            let db_arc = Arc::new(open_db(shard_id)?);
            let mut judge = Judge::new(Some(db_arc.clone()))?;
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
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                let text = String::from_utf8_lossy(&message.data);
                                let parts: Vec<&str> = text.splitn(3, ':').collect();
                                if text.starts_with("TASK:") && parts.len() == 3 {
                                    if let Ok(wasm_bytes) = general_purpose::STANDARD.decode(parts[2]) {
                                        let result = match judge.execute(&wasm_bytes) {
                                            Ok(res) => res.to_string(),
                                            Err(e) => format!("Error: {}", e),
                                        };
                                        let _ = p2p_node.publish_to_topic(&shard_channel, format!("TASK_RESULT:{}:{}", parts[1], result));
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
