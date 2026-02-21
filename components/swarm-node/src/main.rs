use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::{Duration, Instant};
use libp2p::{swarm::SwarmEvent, gossipsub, mdns, identify};
use futures::StreamExt;
use axum::{
    extract::{State, Json, Path}, 
    routing::{post, get}, 
    Router, 
    response::Html, 
    http::StatusCode
};
use std::sync::Arc;
use std::collections::{HashSet, HashMap};
use dashmap::DashMap;
use uuid::Uuid;
use tokio::sync::Mutex;
use serde::Serialize;
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
pub struct SwarmStatus { 
    pub version: String, pub role: String, pub peers_count: usize,
    #[serde(skip)] pub peers: HashSet<libp2p::PeerId>,
}

pub struct JobState {
    pub expected_shards: usize,
    pub results: Vec<(u32, i32)>,
    pub created_at: Instant,
    pub assignments: HashMap<u32, (String, Instant)>,
    pub shards_data: HashMap<u32, Shard>,
}

pub struct AppState {
    pub node_tx: tokio::sync::mpsc::UnboundedSender<String>,
    pub jobs: Arc<DashMap<Uuid, Arc<Mutex<JobState>>>>,
    pub stats: Arc<Mutex<SwarmStatus>>,
    pub health_registry: Arc<DashMap<libp2p::PeerId, u8>>,
    pub pending_dials: Arc<DashMap<libp2p::PeerId, Instant>>,
}

#[derive(Serialize)]
struct JobSubmitResponse {
    job_id: String,
    status: String,
}

#[derive(Serialize)]
struct JobStatusResponse {
    status: String,
    total_sum: i32,
    breakdown: Vec<(u32, i32)>,
    missing_shards: Vec<u32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Gateway { port } => {
            let mut p2p_node = SynapseNode::new(4000).await?;
            let local_peer_id = *p2p_node.swarm.local_peer_id();
            p2p_node.subscribe("swarm-shard-1")?; 
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            
            let jobs = Arc::new(DashMap::new());
            let stats = Arc::new(Mutex::new(SwarmStatus { 
                version: "0.14.0".to_string(), role: "GATEWAY".to_string(), peers_count: 0, peers: HashSet::new(),
            }));
            let health_registry = Arc::new(DashMap::new());
            let pending_dials = Arc::new(DashMap::<libp2p::PeerId, Instant>::new());
            
            let shared_state = Arc::new(AppState { 
                node_tx: tx, jobs: jobs.clone(), stats: stats.clone(), 
                health_registry: health_registry.clone(), pending_dials: pending_dials.clone()
            });
            let (jobs_c, stat_c, health_c, pending_c) = (jobs.clone(), stats.clone(), health_registry.clone(), pending_dials.clone());
            
            tokio::spawn(async move {
                let mut sla_interval = tokio::time::interval(Duration::from_secs(3));
                loop {
                    tokio::select! {
                        _ = sla_interval.tick() => {
                            let mut re_routes = Vec::new();
                            for entry in jobs_c.iter_mut() {
                                let mut job = entry.value().lock().await;
                                for shard_idx in 0..(job.expected_shards as u32) {
                                    if job.results.iter().any(|(s, _)| *s == shard_idx) { continue; }
                                    
                                    let mut needs_reroute = false;
                                    let mut exclude_peer = String::new();

                                    if let Some((worker_id, start_time)) = job.assignments.get(&shard_idx) {
                                        if start_time.elapsed() > Duration::from_secs(15) {
                                            println!("⏰ SLA Timeout: Shard {} on worker {}. Re-routing...", shard_idx, worker_id);
                                            needs_reroute = true;
                                            exclude_peer = worker_id.clone();
                                        }
                                    } else if job.created_at.elapsed() > Duration::from_secs(15) {
                                        println!("⏰ SLA Timeout: Shard {} was never acknowledged. Re-routing...", shard_idx);
                                        needs_reroute = true;
                                    }

                                    if needs_reroute {
                                        if let Some(shard_data) = job.shards_data.get(&shard_idx) {
                                            let mut new_shard = shard_data.clone();
                                            let s = stat_c.lock().await;
                                            
                                            // STRICT EXCLUSION: Find a peer that is NOT the timed-out worker
                                            let idle_peer = s.peers.iter()
                                                .map(|p| p.to_string())
                                                .find(|p| p != &exclude_peer);

                                            new_shard.target_peer = idle_peer.clone();
                                            job.assignments.insert(shard_idx, (idle_peer.unwrap_or_else(|| "unassigned".to_string()), Instant::now()));
                                            re_routes.push(new_shard);
                                        }
                                    }
                                }
                            }
                            for shard in re_routes {
                                let payload = format!("SHARD:{}", serde_json::to_string(&shard).unwrap());
                                let _ = p2p_node.publish_to_topic("swarm-shard-1", payload);
                            }
                        },
                        Some(cmd) = rx.recv() => { let _ = p2p_node.publish_to_topic("swarm-shard-1", cmd); },
                        event = p2p_node.swarm.select_next_some() => {
                            match event {
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::Mdns(mdns::Event::Discovered(list))) => {
                                    for (peer_id, multiaddr) in list {
                                        if !p2p_node.swarm.is_connected(&peer_id) {
                                            if local_peer_id >= peer_id { continue; }
                                            if let Some(last) = pending_c.get(&peer_id) {
                                                if last.elapsed() < Duration::from_secs(5) { continue; }
                                            }
                                            pending_c.insert(peer_id, Instant::now());
                                            p2p_node.swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr.clone());
                                            let _ = p2p_node.swarm.dial(multiaddr);
                                        }
                                    }
                                },
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::Identify(identify::Event::Received { peer_id, info })) => {
                                    println!("🆔 IDENTIFY: Connected to {} ({})", peer_id, info.agent_version);
                                },
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                    let text = String::from_utf8_lossy(&message.data);
                                    if text.starts_with("SHARD_ACK:") {
                                        let parts: Vec<&str> = text.split(':').collect();
                                        if parts.len() >= 4 {
                                            if let (Ok(tid), Ok(s_idx)) = (Uuid::parse_str(parts[1]), parts[2].parse::<u32>()) {
                                                let worker_id = parts[3].to_string();
                                                if let Some(job_ref) = jobs_c.get(&tid) {
                                                    let mut guard = job_ref.value().lock().await;
                                                    guard.assignments.insert(s_idx, (worker_id.clone(), Instant::now()));
                                                    println!("🤝 SHARD_ACK: Worker {} claimed Shard {}", worker_id, s_idx);
                                                }
                                            }
                                        }
                                    } else if text.starts_with("SHARD_RESULT:") {
                                        if let Some(source) = message.source { health_c.remove(&source); }
                                        let parts: Vec<&str> = text.split(':').collect();
                                        if parts.len() >= 4 {
                                            if let (Ok(tid), Ok(s_idx), Ok(val)) = (Uuid::parse_str(parts[1]), parts[2].parse::<u32>(), parts[3].parse::<i32>()) {
                                                if let Some(job_ref) = jobs_c.get(&tid) { 
                                                    let mut guard = job_ref.value().lock().await;
                                                    if !guard.results.iter().any(|(s, _)| *s == s_idx) {
                                                        guard.results.push((s_idx, val));
                                                        println!("📝 Accepted Result: Shard {} = {}", s_idx, val);
                                                    } else {
                                                        println!("⚠️ Dropped Zombie Result: Shard {} = {}", s_idx, val);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                    let mut s = stat_c.lock().await;
                                    if s.peers.insert(peer_id) { s.peers_count = s.peers.len(); println!("📡 CONNECTED: {}", peer_id); }
                                    health_c.remove(&peer_id);
                                    pending_c.remove(&peer_id);
                                },
                                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                                    let mut s = stat_c.lock().await;
                                    if s.peers.remove(&peer_id) { 
                                        s.peers_count = s.peers.len(); 
                                        println!("📉 DISCONNECTED: {}", peer_id); 
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                }
            });

            let app = Router::new()
                .route("/", get(dashboard))
                .route("/api/v1/jobs", post(submit_job))
                .route("/api/v1/jobs/:id", get(get_job_status))
                .with_state(shared_state);
                
            let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
            println!("Gateway Active: http://localhost:{}", port);
            axum::Server::bind(&addr).serve(app.into_make_service()).await?;
        }
        Commands::Start { shard } => {
            let shard_id = shard.unwrap_or(1);
            let port = 4000 + shard_id as u16;
            let mut p2p_node = SynapseNode::new(port).await?;
            let local_peer_id = *p2p_node.swarm.local_peer_id();
            p2p_node.subscribe("swarm-shard-1")?;
            println!("=== Worker Live (Shard {}) on Port {} ===", shard_id, port);
            
            let pending_dials = Arc::new(DashMap::<libp2p::PeerId, Instant>::new());
            let pending_c = pending_dials.clone();

            loop {
                tokio::select! {
                    event = p2p_node.swarm.select_next_some() => {
                        match event {
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Mdns(mdns::Event::Discovered(list))) => {
                                for (peer_id, multiaddr) in list {
                                    if !p2p_node.swarm.is_connected(&peer_id) {
                                        if local_peer_id >= peer_id { continue; }
                                        if let Some(last) = pending_c.get(&peer_id) {
                                            if last.elapsed() < Duration::from_secs(5) { continue; }
                                        }
                                        pending_c.insert(peer_id, Instant::now());
                                        p2p_node.swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr.clone());
                                        let _ = p2p_node.swarm.dial(multiaddr);
                                    }
                                }
                            },
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Identify(identify::Event::Received { peer_id, info })) => {
                                println!("🆔 IDENTIFY: Connected to {} ({})", peer_id, info.agent_version);
                            },
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                let text = String::from_utf8_lossy(&message.data);
                                if text.starts_with("SHARD:") {
                                    if let Ok(shard_data) = serde_json::from_str::<Shard>(&text[6..]) {
                                        let my_id_str = local_peer_id.to_string();
                                        let is_targeted = shard_data.target_peer.as_deref() == Some(my_id_str.as_str());
                                        let is_default = shard_data.target_peer.is_none() && shard_data.shard_index == (shard_id as u32 - 1);

                                        if is_targeted || is_default {
                                            println!("🤝 Claiming Shard {} (Targeted: {})", shard_data.shard_index, is_targeted);
                                            let ack_payload = format!("SHARD_ACK:{}:{}:{}", shard_data.parent_task_id, shard_data.shard_index, my_id_str);
                                            let _ = p2p_node.publish_to_topic("swarm-shard-1", ack_payload);

                                            if let Ok(wasm) = general_purpose::STANDARD.decode(&shard_data.wasm_image) {
                                                println!("⚙️ Executing Shard {}: Processing {} items...", shard_data.shard_index, shard_data.data.len());
                                                
                                                let mut judge = Judge::new(None).unwrap();
                                                let res = match judge.execute(&wasm, &shard_data.data) {
                                                    Ok(val) => val,
                                                    Err(e) => { println!("❌ Wasm Error: {}", e); -1 }
                                                };
                                                
                                                println!("✅ Result: {}", res);
                                                let _ = p2p_node.publish_to_topic("swarm-shard-1", format!("SHARD_RESULT:{}:{}:{}", shard_data.parent_task_id, shard_data.shard_index, res));
                                            }
                                        }
                                    }
                                }
                            },
                            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                println!("📡 CONNECTED: {}", peer_id);
                                pending_c.remove(&peer_id);
                            },
                            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                                println!("📉 DISCONNECTED: {}", peer_id);
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

async fn submit_job(State(state): State<Arc<AppState>>, Json(payload): Json<ShardedDeployRequest>) -> (StatusCode, Json<JobSubmitResponse>) {
    let task_id = Uuid::new_v4();
    let all_peers = state.stats.lock().await.peers.clone();
    let active_peers: Vec<_> = all_peers.iter()
        .filter(|p| *state.health_registry.get(p).as_deref().unwrap_or(&0) < 2)
        .collect();
    
    let peer_count = active_peers.len().max(1); 
    let dataset = payload.dataset;
    let chunk_size = (dataset.len() as f64 / peer_count as f64).ceil() as usize;
    
    let mut shards_data = HashMap::new();

    for (i, chunk) in dataset.chunks(chunk_size).enumerate() {
        let shard = Shard { 
            parent_task_id: task_id, 
            shard_index: i as u32, 
            total_shards: peer_count as u32, 
            data: chunk.to_vec(), 
            wasm_image: payload.wasm_base64.clone(),
            target_peer: None,
        };
        shards_data.insert(i as u32, shard.clone());
        let _ = state.node_tx.send(format!("SHARD:{}", serde_json::to_string(&shard).unwrap()));
    }

    let job_state = JobState {
        expected_shards: peer_count,
        results: Vec::new(),
        created_at: Instant::now(),
        assignments: HashMap::new(),
        shards_data,
    };
    state.jobs.insert(task_id, Arc::new(Mutex::new(job_state)));

    println!("📥 Job Submitted: {} with {} items", task_id, dataset.len());
    
    (StatusCode::ACCEPTED, Json(JobSubmitResponse {
        job_id: task_id.to_string(),
        status: "pending".to_string()
    }))
}

async fn get_job_status(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> (StatusCode, Json<JobStatusResponse>) {
    if let Some(job_ref) = state.jobs.get(&id) {
        let guard = job_ref.lock().await;
        let is_complete = guard.results.len() >= guard.expected_shards;
        
        let sum: i32 = guard.results.iter().map(|(_, val)| val).sum();
        let received_indices: Vec<u32> = guard.results.iter().map(|(idx, _)| *idx).collect();
        let missing_indices: Vec<u32> = (0..guard.expected_shards as u32).filter(|i| !received_indices.contains(i)).collect();

        if is_complete {
            (StatusCode::OK, Json(JobStatusResponse {
                status: "completed".to_string(),
                total_sum: sum,
                breakdown: guard.results.clone(),
                missing_shards: vec![]
            }))
        } else {
            (StatusCode::PARTIAL_CONTENT, Json(JobStatusResponse {
                status: "partial".to_string(),
                total_sum: sum,
                breakdown: guard.results.clone(),
                missing_shards: missing_indices
            }))
        }
    } else {
        (StatusCode::NOT_FOUND, Json(JobStatusResponse {
            status: "not_found".to_string(),
            total_sum: 0,
            breakdown: vec![],
            missing_shards: vec![]
        }))
    }
}

async fn dashboard(State(_state): State<Arc<AppState>>) -> Html<String> {
    Html("<h1>Please use API endpoints for v0.14.0</h1>".to_string())
}
