use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::{Duration, Instant};
use libp2p::{swarm::SwarmEvent, mdns, request_response};
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

use synapse::{SynapseNode, SynapseBehaviorEvent, SwarmRequest, SwarmResponse};
use judge::Judge;

mod sharding;
use sharding::{Shard, ShardResult, ShardedDeployRequest};

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
    pub assignments: HashMap<u32, (libp2p::PeerId, Instant)>,
    pub shards_data: HashMap<u32, Shard>,
}

pub enum NodeCommand {
    Unicast(libp2p::PeerId, SwarmRequest),
}

pub struct AppState {
    pub node_tx: tokio::sync::mpsc::UnboundedSender<NodeCommand>,
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
            p2p_node.subscribe("swarm-control-plane")?; 
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<NodeCommand>();
            
            let jobs = Arc::new(DashMap::new());
            let stats = Arc::new(Mutex::new(SwarmStatus { 
                version: "0.15.0".to_string(), role: "GATEWAY".to_string(), peers_count: 0, peers: HashSet::new(),
            }));
            let health_registry = Arc::new(DashMap::new());
            let pending_dials = Arc::new(DashMap::<libp2p::PeerId, Instant>::new());
            
            let shared_state = Arc::new(AppState { 
                node_tx: tx.clone(), jobs: jobs.clone(), stats: stats.clone(), 
                health_registry: health_registry.clone(), pending_dials: pending_dials.clone()
            });
            let (jobs_c, stat_c, health_c, pending_c) = (jobs.clone(), stats.clone(), health_registry.clone(), pending_dials.clone());
            
            tokio::spawn(async move {
                let mut sla_interval = tokio::time::interval(Duration::from_secs(3));
                loop {
                    tokio::select! {
                        _ = sla_interval.tick() => {
                            let mut re_routes = Vec::new();
                            let mut jobs_to_prune = Vec::new();

                            for entry in jobs_c.iter_mut() {
                                let mut job = entry.value().lock().await;
                                let job_id = *entry.key();
                                let is_complete = job.results.len() >= job.expected_shards;

                                if is_complete {
                                    if job.created_at.elapsed() > Duration::from_secs(300) { jobs_to_prune.push(job_id); }
                                    continue;
                                }

                                for shard_idx in 0..(job.expected_shards as u32) {
                                    if job.results.iter().any(|(s, _)| *s == shard_idx) { continue; }
                                    
                                    let mut needs_reroute = false;
                                    let mut exclude_peer = None;

                                    if let Some((worker_id, start_time)) = job.assignments.get(&shard_idx) {
                                        if start_time.elapsed() > Duration::from_secs(15) {
                                            println!("⏰ SLA Timeout: Shard {} on worker {}. Re-routing...", shard_idx, worker_id);
                                            needs_reroute = true;
                                            exclude_peer = Some(*worker_id);
                                        }
                                    } else if job.created_at.elapsed() > Duration::from_secs(15) {
                                        println!("⏰ SLA Timeout: Shard {} was never acknowledged. Re-routing...", shard_idx);
                                        needs_reroute = true;
                                    }

                                    if needs_reroute {
                                        // Decoupled the immutable borrow from the job struct!
                                        let shard_data_opt = job.shards_data.get(&shard_idx).cloned();
                                        
                                        if let Some(shard_data) = shard_data_opt {
                                            let s = stat_c.lock().await;
                                            let idle_peer = s.peers.iter().find(|&&p| Some(p) != exclude_peer).copied();
                                            
                                            if let Some(peer) = idle_peer {
                                                job.assignments.insert(shard_idx, (peer, Instant::now()));
                                                re_routes.push((peer, shard_data));
                                            } else {
                                                println!("⚠️ No alternate peers available for re-route!");
                                            }
                                        }
                                    }
                                }
                            }

                            for id in jobs_to_prune {
                                jobs_c.remove(&id);
                                println!("🧹 GC: Pruned completed job {}.", id);
                            }

                            for (peer, shard) in re_routes {
                                let req = SwarmRequest::DispatchShard(serde_json::to_string(&shard).unwrap());
                                let _ = tx.send(NodeCommand::Unicast(peer, req));
                            }
                        },
                        Some(cmd) = rx.recv() => { 
                            match cmd {
                                NodeCommand::Unicast(peer, req) => { let _ = p2p_node.send_request(&peer, req); }
                            }
                        },
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
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::ReqRes(request_response::Event::Message { peer, message })) => {
                                    match message {
                                        request_response::Message::Request { request, channel, .. } => {
                                            if let SwarmRequest::SubmitResult(json_payload) = request {
                                                // 1. Send Decoupled ACK instantly
                                                p2p_node.send_response(channel, SwarmResponse::Ack);
                                                
                                                // 2. Process Result
                                                if let Ok(res_data) = serde_json::from_str::<ShardResult>(&json_payload) {
                                                    health_c.remove(&peer);
                                                    if let Some(job_ref) = jobs_c.get(&res_data.job_id) { 
                                                        let mut guard = job_ref.value().lock().await;
                                                        if !guard.results.iter().any(|(s, _)| *s == res_data.shard_index) {
                                                            guard.results.push((res_data.shard_index, res_data.result));
                                                            println!("📝 Accepted Result: Shard {} = {}", res_data.shard_index, res_data.result);
                                                        } else {
                                                            println!("⚠️ Dropped Zombie Result: Shard {} = {}", res_data.shard_index, res_data.result);
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        request_response::Message::Response { .. } => {
                                            // Worker ACKed our Dispatch request. SLA timer is already tracking them.
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
            p2p_node.subscribe("swarm-control-plane")?;
            println!("=== Worker Live (Shard {}) on Port {} ===", shard_id, port);
            
            let pending_dials = Arc::new(DashMap::<libp2p::PeerId, Instant>::new());
            let pending_c = pending_dials.clone();
            
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<NodeCommand>();

            loop {
                tokio::select! {
                    Some(cmd) = rx.recv() => {
                        match cmd {
                            NodeCommand::Unicast(peer, req) => { let _ = p2p_node.send_request(&peer, req); }
                        }
                    },
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
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::ReqRes(request_response::Event::Message { peer, message })) => {
                                match message {
                                    request_response::Message::Request { request, channel, .. } => {
                                        if let SwarmRequest::DispatchShard(json_payload) = request {
                                            // 1. Instantly ACK to free up the TCP stream
                                            p2p_node.send_response(channel, SwarmResponse::Ack);
                                            
                                            // 2. Offload execution to a background task
                                            if let Ok(shard_data) = serde_json::from_str::<Shard>(&json_payload) {
                                                println!("🤝 Unicast Received: Claiming Shard {}", shard_data.shard_index);
                                                
                                                let worker_tx = tx.clone();
                                                let gateway_id = peer;
                                                
                                                tokio::spawn(async move {
                                                    if let Ok(wasm) = general_purpose::STANDARD.decode(&shard_data.wasm_image) {
                                                        println!("⚙️ Executing Shard {}: Processing {} items...", shard_data.shard_index, shard_data.data.len());
                                                        
                                                        let mut judge = Judge::new(None).unwrap();
                                                        let res = match judge.execute(&wasm, &shard_data.data) {
                                                            Ok(val) => val,
                                                            Err(e) => { println!("❌ Wasm Error: {}", e); -1 }
                                                        };
                                                        
                                                        println!("✅ Result: {}", res);
                                                        
                                                        let result_obj = ShardResult {
                                                            job_id: shard_data.parent_task_id,
                                                            shard_index: shard_data.shard_index,
                                                            result: res,
                                                        };
                                                        let req = SwarmRequest::SubmitResult(serde_json::to_string(&result_obj).unwrap());
                                                        let _ = worker_tx.send(NodeCommand::Unicast(gateway_id, req));
                                                    }
                                                });
                                            }
                                        }
                                    },
                                    request_response::Message::Response { .. } => {
                                        // Gateway ACKed our Result. We are done!
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
    let all_peers: Vec<_> = state.stats.lock().await.peers.iter().copied().collect();
    let active_peers: Vec<_> = all_peers.into_iter()
        .filter(|p| *state.health_registry.get(p).as_deref().unwrap_or(&0) < 2)
        .collect();
    
    let peer_count = active_peers.len().max(1); 
    let dataset = payload.dataset;
    let chunk_size = (dataset.len() as f64 / peer_count as f64).ceil() as usize;
    
    let mut shards_data = HashMap::new();
    let mut assignments = HashMap::new();

    for (i, chunk) in dataset.chunks(chunk_size).enumerate() {
        let target_peer = active_peers.get(i % active_peers.len()).copied();
        
        let shard = Shard { 
            parent_task_id: task_id, 
            shard_index: i as u32, 
            total_shards: peer_count as u32, 
            data: chunk.to_vec(), 
            wasm_image: payload.wasm_base64.clone(),
            target_peer: target_peer.map(|p| p.to_string()),
        };
        
        shards_data.insert(i as u32, shard.clone());
        
        if let Some(peer) = target_peer {
            assignments.insert(i as u32, (peer, Instant::now()));
            let req = SwarmRequest::DispatchShard(serde_json::to_string(&shard).unwrap());
            let _ = state.node_tx.send(NodeCommand::Unicast(peer, req));
        }
    }

    let job_state = JobState {
        expected_shards: peer_count,
        results: Vec::new(),
        created_at: Instant::now(),
        assignments,
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
    Html(r#"
        <div style="font-family: sans-serif; padding: 2rem;">
            <h1>🐝 Swarm Runtime Gateway</h1>
            <p><strong>Version:</strong> v0.15.0 (Unicast Data Plane)</p>
            <hr>
            <h3>Available API Endpoints:</h3>
            <ul>
                <li><strong>POST</strong> <code>/api/v1/jobs</code> - Submit a new Wasm job</li>
                <li><strong>GET</strong> <code>/api/v1/jobs/:id</code> - Check job status and results</li>
            </ul>
        </div>
    "#.to_string())
}
