use std::collections::HashSet;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::Duration;
use libp2p::swarm::SwarmEvent;
use libp2p::gossipsub;
use futures::StreamExt;
use axum::{extract::{State, Json}, routing::post, Router, http::StatusCode};
use std::sync::Arc;
use dashmap::DashMap;
use uuid::Uuid;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};

use synapse::{SynapseNode, SynapseBehaviorEvent};
use judge::Judge;

mod sharding;
use sharding::{Shard, ShardedDeployRequest};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start { #[arg(long)] shard: Option<u64> },
    Gateway { #[arg(long, default_value = "3000")] port: u16 },
}

#[derive(Serialize, Clone)]
struct SwarmStatus { version: String, role: String,
    pub peers_count: usize, #[serde(skip)]
    pub peers: HashSet<libp2p::PeerId> }

struct AppState {
    pub node_tx: tokio::sync::mpsc::UnboundedSender<String>,
    pub aggregations: Arc<DashMap<Uuid, Arc<Mutex<Vec<i32>>>>>,
    pub stats: Arc<Mutex<SwarmStatus>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Gateway { port } => {
            // Gateway always sits on P2P port 4000
            let mut p2p_node = SynapseNode::new(4000).await?;
            p2p_node.subscribe("swarm-shard-1")?; 
            
            // Auto-probe local ports for workers
            for p in 4001..4005 {
                let _ = p2p_node.swarm.dial(format!("/ip4/127.0.0.1/tcp/{}", p).parse::<libp2p::Multiaddr>()?);
            }

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            let aggregations = Arc::new(DashMap::new());
            let stats = Arc::new(Mutex::new(SwarmStatus { version: "0.8.0".to_string(), role: "Orchestrator".to_string(), peers_count: 0, peers: HashSet::new() }));

            let shared_state = Arc::new(AppState {
                node_tx: tx,
                aggregations: aggregations.clone(),
                stats: stats.clone(),
            });

            let agg_clone = aggregations.clone();
            let stats_clone = stats.clone();
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        Some(cmd) = rx.recv() => { let _ = p2p_node.publish_to_topic("swarm-shard-1", cmd); },
                        event = p2p_node.swarm.select_next_some() => {
                            match event {
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                    let text = String::from_utf8_lossy(&message.data);
                                    if text.starts_with("SHARD_RESULT:") {
                                        let parts: Vec<&str> = text.split(':').collect();
                                        if parts.len() >= 4 {
                                            if let Ok(tid) = Uuid::parse_str(parts[1]) {
                                                if let Ok(val) = parts[3].parse::<i32>() {
                                                    if let Some(agg_ref) = agg_clone.get(&tid) {
                                                        agg_ref.value().lock().await.push(val);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::Mdns(mdns_event)) => {
                                    use libp2p::mdns::Event;
                                    let mut s = stats_clone.lock().await;
                                    match mdns_event {
                                        Event::Discovered(list) => {
                                            for (peer_id, _) in list {
                                                p2p_node.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                                s.peers.insert(peer_id); s.peers_count = s.peers.len();
                                                println!("Orchestrator: Found Peer {}!", peer_id);
                                            }
                                        },
                                        _ => {}
                                    }
                                },
                                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                    let mut s = stats_clone.lock().await;
                                    s.peers.insert(peer_id); s.peers_count = s.peers.len();
                                    p2p_node.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                    println!("Orchestrator: Connection established with {}!", peer_id);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            });

            let app = Router::new()
                .route("/deploy/shard", post(|State(state): State<Arc<AppState>>, Json(payload): Json<ShardedDeployRequest>| async move {
                    let task_id = Uuid::new_v4();
                    let peer_count = state.stats.lock().await.peers.len().max(1);
                    state.aggregations.insert(task_id, Arc::new(Mutex::new(Vec::new())));

                    println!("Orchestrator: Splitting task across {} peers", peer_count);
                    let chunk_size = payload.range_size / peer_count as u64;
                    for i in 0..peer_count {
                        let start = i as u64 * chunk_size;
                        let end = if i == (peer_count - 1) as usize { payload.range_size } else { (i as u64 + 1) * chunk_size };
                        let shard = Shard { parent_task_id: task_id, shard_index: i as u32, total_shards: peer_count as u32, data_range_start: start, data_range_end: end, wasm_image: payload.wasm_base64.clone() };
                        let _ = state.node_tx.send(format!("SHARD:{}", serde_json::to_string(&shard).unwrap()));
                    }

                    for _ in 0..100 {
                        if let Some(agg_ref) = state.aggregations.get(&task_id) {
                            let results = agg_ref.value().lock().await;
                            if results.len() >= peer_count {
                                let sum: i32 = results.iter().sum();
                                return (StatusCode::OK, format!("SUCCESS: Parallel Sum = {}\n", sum));
                            }
                        }
                        tokio::time::sleep(Duration::from_millis(200)).await;
                    }
                    (StatusCode::GATEWAY_TIMEOUT, "Timeout waiting for shards\n".to_string())
                }))
                .with_state(shared_state);

            let addr = std::net::SocketAddr::from(([0, 0, 0, 0], *port));
            println!("Gateway: Listening on http://{}", addr);
            axum::Server::bind(&addr).serve(app.into_make_service()).await?;
        }
        Commands::Start { shard, .. } => {
            let shard_id = shard.unwrap_or(1);
            let mut p2p_node = SynapseNode::new(4000 + shard_id as u16).await?;
            p2p_node.subscribe("swarm-shard-1")?;
            
            // Workers dial the Gateway at 4000
            let _ = p2p_node.swarm.dial("/ip4/127.0.0.1/tcp/4000".parse::<libp2p::Multiaddr>()?);

            let db = Arc::new(sled::open(format!("storage/shard_{}", shard_id))?);
            let mut judge = Judge::new(Some(db))?;
            println!("=== Worker Live (Shard {}) ===", shard_id);
            loop {
                let event = p2p_node.swarm.select_next_some().await;
                if let SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) = event {
                    let text = String::from_utf8_lossy(&message.data);
                    if text.starts_with("SHARD:") {
                        if let Ok(shard_data) = serde_json::from_str::<Shard>(&text[6..]) {
                            if let Ok(wasm) = general_purpose::STANDARD.decode(&shard_data.wasm_image) {
                                println!("Worker: Computing range [{}..{}]", shard_data.data_range_start, shard_data.data_range_end);
                                let res = judge.execute(&wasm, Some((shard_data.data_range_start, shard_data.data_range_end))).unwrap_or(-1);
                                let _ = p2p_node.publish_to_topic("swarm-shard-1", format!("SHARD_RESULT:{}:{}:{}", shard_data.parent_task_id, shard_data.shard_index, res));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
