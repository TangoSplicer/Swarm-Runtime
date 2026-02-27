use anyhow::Result;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use libp2p::{swarm::SwarmEvent, mdns, request_response, gossipsub};
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
use ed25519_dalek::{SigningKey, Signer};

use synapse::{SynapseNode, SynapseBehaviorEvent, SwarmRequest, SwarmResponse};
use crate::types::*;

pub async fn run_gateway(port: u16, signing_key: SigningKey) -> Result<()> {
    let mut p2p_node = SynapseNode::new(4000).await?;
    let local_peer_id = *p2p_node.swarm.local_peer_id();
    p2p_node.subscribe("swarm-control-plane")?; 
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<NodeCommand>();
    
    let jobs = Arc::new(DashMap::new());
    let stats = Arc::new(Mutex::new(SwarmStatus { 
        version: "0.19.0".to_string(), role: "GATEWAY".to_string(), peers_count: 0, peers: HashSet::new(),
    }));
    let health_registry = Arc::new(DashMap::new());
    let pending_dials = Arc::new(DashMap::<libp2p::PeerId, Instant>::new());
    let telemetry_registry = Arc::new(DashMap::<libp2p::PeerId, Telemetry>::new());
    
    let shared_state = Arc::new(AppState { 
        node_tx: tx.clone(), jobs: jobs.clone(), stats: stats.clone(), 
        health_registry: health_registry.clone(), pending_dials: pending_dials.clone(),
        telemetry_registry: telemetry_registry.clone(),
        signing_key: signing_key.clone(),
    });
    let (jobs_c, stat_c, health_c, pending_c, tel_c) = (jobs.clone(), stats.clone(), health_registry.clone(), pending_dials.clone(), telemetry_registry.clone());
    
    tokio::spawn(async move {
        let mut sla_interval = tokio::time::interval(Duration::from_secs(3));
        loop {
            tokio::select! {
                _ = sla_interval.tick() => {
                    let mut re_routes = Vec::new();
                    let mut new_dispatches = Vec::new();
                    let mut jobs_to_prune = Vec::new();

                    for entry in jobs_c.iter_mut() {
                        let mut job = entry.value().lock().await;
                        let job_id = *entry.key();
                        
                        if let Some(dataset) = job.unassigned_dataset.take() {
                            let s = stat_c.lock().await;
                            let active_peers: Vec<_> = s.peers.iter().copied().collect();

                            if active_peers.is_empty() || tel_c.is_empty() {
                                job.unassigned_dataset = Some(dataset); 
                                continue;
                            }

                            job.redundancy = 2.min(active_peers.len());
                            println!("🧠 SCHEDULER: Optimizing workload with Redundancy Factor {}...", job.redundancy);
                            
                            let mut peer_fitness = Vec::new();
                            let mut total_fitness = 0.0;
                            for peer in &active_peers {
                                let fitness = if let Some(tel) = tel_c.get(peer) {
                                    (tel.free_ram_mb as f32) / (tel.cpu_usage + 1.0)
                                } else { 1.0 };
                                peer_fitness.push((*peer, fitness));
                                total_fitness += fitness;
                            }

                            peer_fitness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                            job.expected_shards = active_peers.len();
                            let mut start_idx = 0;

                            for (i, (primary_peer, fitness)) in peer_fitness.iter().enumerate() {
                                if start_idx >= dataset.len() { break; } 
                                
                                let weight = fitness / total_fitness;
                                let mut chunk_size = (dataset.len() as f32 * weight).floor() as usize;

                                if chunk_size == 0 && dataset.len() >= active_peers.len() { chunk_size = 1; }
                                if i == active_peers.len() - 1 { chunk_size = dataset.len() - start_idx; }
                                if start_idx + chunk_size > dataset.len() { chunk_size = dataset.len() - start_idx; }

                                let chunk = &dataset[start_idx..start_idx + chunk_size];
                                start_idx += chunk_size;

                                let shard = Shard {
                                    parent_task_id: job_id,
                                    shard_index: i as u32,
                                    total_shards: active_peers.len() as u32,
                                    data: chunk.to_vec(),
                                    wasm_image: job.wasm_image.clone(),
                                    target_peer: Some(primary_peer.to_string()),
                                };

                                let mut peer_assignments = HashMap::new();
                                
                                peer_assignments.insert(*primary_peer, Instant::now());
                                new_dispatches.push((*primary_peer, shard.clone()));

                                if job.redundancy > 1 {
                                    let secondary_peer = peer_fitness[(i + 1) % peer_fitness.len()].0;
                                    peer_assignments.insert(secondary_peer, Instant::now());
                                    new_dispatches.push((secondary_peer, shard.clone()));
                                }

                                job.shards_data.insert(i as u32, shard.clone());
                                job.assignments.insert(i as u32, peer_assignments);
                            }
                            continue; 
                        }

                        let is_complete = job.verified_results.len() >= job.expected_shards;
                        if is_complete {
                            if job.created_at.elapsed() > Duration::from_secs(300) { jobs_to_prune.push(job_id); }
                            continue;
                        }

                        for shard_idx in 0..(job.expected_shards as u32) {
                            if job.verified_results.contains_key(&shard_idx) { continue; }
                            let mut failed_peers = Vec::new();

                            if let Some(peer_assignments) = job.assignments.get(&shard_idx) {
                                for (peer, start_time) in peer_assignments {
                                    if start_time.elapsed() > Duration::from_secs(15) {
                                        let has_result = job.raw_results.get(&shard_idx).map_or(false, |r| r.contains_key(peer));
                                        if !has_result {
                                            println!("⏰ SLA Timeout: Peer {} dropped Shard {}. Re-routing...", peer, shard_idx);
                                            failed_peers.push(*peer);
                                        }
                                    }
                                }
                            }

                            for failed_peer in failed_peers {
                                if let Some(shard_data) = job.shards_data.get(&shard_idx).cloned() {
                                    let s = stat_c.lock().await;
                                    let current_assignees: Vec<_> = job.assignments.get(&shard_idx).unwrap().keys().cloned().collect();
                                    let idle_peer = s.peers.iter().find(|&&p| !current_assignees.contains(&p)).copied();

                                    if let Some(new_peer) = idle_peer {
                                        let assignments = job.assignments.get_mut(&shard_idx).unwrap();
                                        assignments.remove(&failed_peer);
                                        assignments.insert(new_peer, Instant::now());
                                        re_routes.push((new_peer, shard_data));
                                    }
                                }
                            }
                        }
                    }

                    for id in jobs_to_prune {
                        jobs_c.remove(&id);
                        println!("🧹 GC: Pruned completed job {}.", id);
                    }

                    for (peer, shard) in new_dispatches.into_iter().chain(re_routes) {
                        let payload_json = serde_json::to_string(&shard).unwrap();
                        let expires_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 60;
                        let message_to_sign = format!("{}:{}", payload_json, expires_at);
                        let signature = signing_key.sign(message_to_sign.as_bytes());

                        let signed_payload = SignedPayload { payload_json, expires_at, signature: signature.to_bytes().to_vec() };
                        let req = SwarmRequest::DispatchShard(serde_json::to_string(&signed_payload).unwrap());
                        let _ = tx.send(NodeCommand::Unicast(peer, req));
                    }
                },
                Some(cmd) = rx.recv() => { 
                    match cmd {
                        NodeCommand::Unicast(peer, req) => { let _ = p2p_node.send_request(&peer, req); },
                        NodeCommand::Broadcast(msg) => { let _ = p2p_node.publish_to_topic("swarm-control-plane", msg); },
                        NodeCommand::Disconnect(peer) => { let _ = p2p_node.swarm.disconnect_peer_id(peer); },
                        // FIX: Exhaustive match requirement. Gateway gracefully ignores file pins.
                        NodeCommand::PinFile(_) => {} 
                    }
                },
                event = p2p_node.swarm.select_next_some() => {
                    match event {
                        SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                            let text = String::from_utf8_lossy(&message.data);
                            if text.starts_with("TEL:") {
                                if let Ok(tel) = serde_json::from_str::<Telemetry>(&text[4..]) {
                                    if let Ok(peer) = tel.peer_id.parse::<libp2p::PeerId>() {
                                        tel_c.insert(peer, tel);
                                    }
                                }
                            }
                        },
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
                            if let request_response::Message::Request { request: SwarmRequest::SubmitResult(json_payload), channel, .. } = message {
                                p2p_node.send_response(channel, SwarmResponse::Ack);
                                
                                if let Ok(res_data) = serde_json::from_str::<ShardResult>(&json_payload) {
                                    health_c.remove(&peer);
                                    if let Some(job_ref) = jobs_c.get(&res_data.job_id) { 
                                        let mut guard = job_ref.value().lock().await;
                                        
                                        if guard.verified_results.contains_key(&res_data.shard_index) { continue; }

                                        let is_consensus_ready = {
                                            let shard_results = guard.raw_results.entry(res_data.shard_index).or_insert_with(HashMap::new);
                                            shard_results.insert(peer, (res_data.result, res_data.result_hash.clone()));
                                            
                                            let display_hash = if res_data.result_hash.len() >= 8 { &res_data.result_hash[..8] } else { &res_data.result_hash };
                                            println!("📩 Received Result Hash: [{}] from {}", display_hash, peer);
                                            
                                            shard_results.len() >= guard.redundancy
                                        };

                                        if is_consensus_ready {
                                            if let Some(shard_results) = guard.raw_results.remove(&res_data.shard_index) {
                                                let mut unique_results: Vec<(i32, String)> = shard_results.values().cloned().collect();
                                                unique_results.dedup();

                                                if unique_results.len() == 1 {
                                                    let (final_result, final_hash) = &unique_results[0];
                                                    
                                                    let final_display = if final_hash.len() >= 8 { &final_hash[..8] } else { final_hash };
                                                    println!("✅ HASH CONSENSUS REACHED: Shard {} state verified [{}]", res_data.shard_index, final_display);
                                                    
                                                    guard.verified_results.insert(res_data.shard_index, (*final_result, final_hash.clone()));
                                                } else {
                                                    println!("🚨 HASH COLLISION DETECTED! State mutation mismatch on Shard {}.", res_data.shard_index);
                                                    for (p, _) in shard_results.iter() {
                                                        let _ = tx.send(NodeCommand::Disconnect(*p));
                                                    }
                                                    guard.assignments.remove(&res_data.shard_index);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            let mut s = stat_c.lock().await;
                            if s.peers.insert(peer_id) { s.peers_count = s.peers.len(); println!("📡 CONNECTED: {}", peer_id); }
                            pending_c.remove(&peer_id);
                        },
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            let mut s = stat_c.lock().await;
                            if s.peers.remove(&peer_id) { 
                                s.peers_count = s.peers.len(); 
                                println!("📉 DISCONNECTED: {}", peer_id); 
                            }
                            tel_c.remove(&peer_id);
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
    
    Ok(())
}

async fn submit_job(State(state): State<Arc<AppState>>, Json(payload): Json<ShardedDeployRequest>) -> (StatusCode, Json<JobSubmitResponse>) {
    let task_id = Uuid::new_v4();
    
    let job_state = JobState {
        expected_shards: 0, 
        redundancy: 1, 
        raw_results: HashMap::new(),
        verified_results: HashMap::new(),
        created_at: Instant::now(),
        assignments: HashMap::new(),
        shards_data: HashMap::new(),
        unassigned_dataset: Some(payload.dataset.clone()),
        wasm_image: payload.wasm_base64,
    };
    
    state.jobs.insert(task_id, Arc::new(Mutex::new(job_state)));

    println!("📥 Job Queued: {} with {} Complex Items", task_id, payload.dataset.len());
    
    (StatusCode::ACCEPTED, Json(JobSubmitResponse {
        job_id: task_id.to_string(),
        status: "pending_scheduler".to_string()
    }))
}

async fn get_job_status(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> (StatusCode, Json<JobStatusResponse>) {
    if let Some(job_ref) = state.jobs.get(&id) {
        let guard = job_ref.lock().await;
        
        if guard.unassigned_dataset.is_some() {
            return (StatusCode::ACCEPTED, Json(JobStatusResponse {
                status: "awaiting_telemetry".to_string(), total_sum: 0, breakdown: vec![], hashes: vec![], missing_shards: vec![]
            }));
        }

        let is_complete = guard.verified_results.len() >= guard.expected_shards;
        let sum: i32 = guard.verified_results.values().map(|(res, _)| *res).sum();
        
        let breakdown: Vec<(u32, i32)> = guard.verified_results.iter().map(|(k, v)| (*k, v.0)).collect();
        let hashes: Vec<(u32, String)> = guard.verified_results.iter().map(|(k, v)| (*k, v.1.clone())).collect();
        
        let received_indices: Vec<u32> = guard.verified_results.keys().copied().collect();
        let missing_indices: Vec<u32> = (0..guard.expected_shards as u32).filter(|i| !received_indices.contains(i)).collect();

        if is_complete {
            (StatusCode::OK, Json(JobStatusResponse { status: "completed".to_string(), total_sum: sum, breakdown, hashes, missing_shards: vec![] }))
        } else {
            (StatusCode::PARTIAL_CONTENT, Json(JobStatusResponse { status: "partial".to_string(), total_sum: sum, breakdown, hashes, missing_shards: missing_indices }))
        }
    } else {
        (StatusCode::NOT_FOUND, Json(JobStatusResponse { status: "not_found".to_string(), total_sum: 0, breakdown: vec![], hashes: vec![], missing_shards: vec![] }))
    }
}

async fn dashboard(State(_state): State<Arc<AppState>>) -> Html<String> {
    Html(r#"
        <div style="font-family: sans-serif; padding: 2rem;">
            <h1>🐝 Swarm Runtime Gateway</h1>
            <p><strong>Version:</strong> v0.19.0 (WASI Virtual File System)</p>
        </div>
    "#.to_string())
}
