use crate::types::*;
use anyhow::Result;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router,
};
use dashmap::DashMap;
use ed25519_dalek::{Signer, SigningKey};
use futures::StreamExt;
use libp2p::{gossipsub, request_response, swarm::SwarmEvent};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use synapse::{SwarmRequest, SwarmResponse, SynapseBehaviorEvent, SynapseNode};
use tokio::sync::Mutex;
use uuid::Uuid;

pub async fn run_gateway(port: u16, signing_key: SigningKey) -> Result<()> {
    let mut p2p_node = SynapseNode::new(4000, signing_key.to_bytes()).await?;
    let _local_peer_id = *p2p_node.swarm.local_peer_id();
    p2p_node.subscribe("swarm-control-plane")?;
    p2p_node.subscribe("swarm-gateway-sync")?;
    let (tx, mut rx) = tokio::sync::mpsc::channel::<NodeCommand>(1000);

    let jobs = Arc::new(DashMap::new());
    let stats = Arc::new(Mutex::new(SwarmStatus {
        version: "0.26.0".to_string(),
        role: "GATEWAY".to_string(),
        peers_count: 0,
        peers: HashSet::new(),
    }));
    let health_registry = Arc::new(DashMap::new());
    let pending_dials = Arc::new(DashMap::<libp2p::PeerId, Instant>::new());
    let telemetry_registry = Arc::new(DashMap::<libp2p::PeerId, Telemetry>::new());

    let contract_states = Arc::new(DashMap::<String, String>::new());

    let shared_state = Arc::new(AppState {
        node_tx: tx.clone(),
        jobs: jobs.clone(),
        stats: stats.clone(),
        health_registry: health_registry.clone(),
        pending_dials: pending_dials.clone(),
        telemetry_registry: telemetry_registry.clone(),
        signing_key: signing_key.clone(),
    });

    let (jobs_c, stat_c, health_c, pending_c, tel_c, contract_states_c) = (
        jobs.clone(),
        stats.clone(),
        health_registry.clone(),
        pending_dials.clone(),
        telemetry_registry.clone(),
        contract_states.clone(),
    );

    tokio::spawn(async move {
        let mut sla_interval = tokio::time::interval(Duration::from_secs(3));
        let mut req_to_hash: HashMap<libp2p::request_response::OutboundRequestId, String> =
            HashMap::new();
        let mut hash_to_tx: HashMap<String, tokio::sync::oneshot::Sender<Option<Vec<u8>>>> =
            HashMap::new();

        loop {
            tokio::select! {
                            _ = sla_interval.tick() => {
                                let mut re_routes = Vec::new();
                                let mut new_dispatches = Vec::new();
                                let mut jobs_to_prune = Vec::new();

                                let active_peers: Vec<_> = stat_c.lock().await.peers.iter().copied().collect();
                                let jobs_snapshot: Vec<(Uuid, Arc<Mutex<JobState>>)> = jobs_c.iter().map(|entry| (*entry.key(), entry.value().clone())).collect();

                                for (job_id, job_arc) in jobs_snapshot {
                                    let mut job = job_arc.lock().await;

                                    if let Some(dataset) = job.unassigned_dataset.take() {
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

                                        let is_stateful = !job.wasm_image.is_empty() && job.wasm_image != b"NONE";

                                        if is_stateful {
                                            job.expected_shards = 1;

                                            let mut hasher = Sha256::new();
                                            hasher.update(&job.wasm_image);
                                            let contract_key = hex::encode(hasher.finalize());

                                            let mut out_wasm_image = job.wasm_image.clone();
                                            if let Some(latest_state) = contract_states_c.get(&contract_key) {
                                                let mut final_wasm = out_wasm_image.clone(); final_wasm.extend_from_slice(b"|STATE:"); final_wasm.extend_from_slice(latest_state.value().as_bytes()); out_wasm_image = final_wasm;
                                            }

                                            let primary_peer = peer_fitness[0].0;
                                            let shard = Shard {
                                                parent_task_id: job_id,
                                                shard_index: 0,
                                                total_shards: 1,
                                                data: dataset.clone(),
                                                wasm_image: out_wasm_image,
                                                target_peer: Some(primary_peer.to_string()),
                                            };

                                            let mut peer_assignments = HashMap::new();
                                            peer_assignments.insert(primary_peer, Instant::now());
                                            new_dispatches.push((primary_peer, shard.clone()));

                                            if job.redundancy > 1 && peer_fitness.len() > 1 {
                                                let secondary_peer = peer_fitness[1].0;
                                                peer_assignments.insert(secondary_peer, Instant::now());
                                                new_dispatches.push((secondary_peer, shard.clone()));
                                            }

                                            job.shards_data.insert(0, shard.clone());
                                            job.assignments.insert(0, peer_assignments);

                                        } else {
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
                                                let current_assignees: HashSet<_> = job.assignments.get(&shard_idx).unwrap().keys().cloned().collect();
                                                let idle_peer = active_peers.iter().find(|&&p| !current_assignees.contains(&p)).copied();

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

                                    if let Err(e) = tx.try_send(NodeCommand::Unicast(peer, req)) {
                                        eprintln!("⚠️ BACKPRESSURE ALARM: Failed to send DispatchShard: {}", e);
                                    }
                                }
                            },
                            Some(cmd) = rx.recv() => {
                                match cmd {
                                    NodeCommand::Unicast(peer, req) => { let _ = p2p_node.send_request(&peer, req); },
                                    NodeCommand::Broadcast(msg) => { let _ = p2p_node.publish_to_topic("swarm-control-plane", msg); },
                                    NodeCommand::GatewaySync(msg) => { let _ = p2p_node.publish_to_topic("swarm-gateway-sync", msg); },
                                    NodeCommand::Disconnect(peer) => { let _ = p2p_node.swarm.disconnect_peer_id(peer); },
                                    NodeCommand::PinFile(_) => {}
                                    NodeCommand::FetchFile(hash, reply_tx) => {
                                        let peers = stat_c.lock().await.peers.clone();
                                        hash_to_tx.insert(hash.clone(), reply_tx);
                                        for peer in peers {
                                            let req_id = p2p_node.send_request(&peer, SwarmRequest::FetchData(hash.clone()));
                                            req_to_hash.insert(req_id, hash.clone());
                                        }
                                    }
                                }
                            },
                            event = p2p_node.swarm.select_next_some() => {
                                match event {
                                    SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                        let text = String::from_utf8_lossy(&message.data);
                                        if text.starts_with("SYNC_STATE:") {
                                            let payload = text[11..].to_string();
                                            let cs_clone = contract_states_c.clone();
                                            tokio::spawn(async move {
                                                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&payload) {
                                                    if let (Some(k), Some(v)) = (parsed.get("k").and_then(|v| v.as_str()), parsed.get("v").and_then(|v| v.as_str())) {
                                                        cs_clone.insert(k.to_string(), v.to_string());
                                                        println!("🔄 FEDERATION: Synced State Hash [{}] from Peer Gateway", &v[..8]);
                                                    }
                                                }
                                            });
                                        }
                                        if text.starts_with("TEL:") {
                                            if let Ok(tel) = serde_json::from_str::<Telemetry>(&text[4..]) {
                                                if let Ok(peer) = tel.peer_id.parse::<libp2p::PeerId>() {
                                                    tel_c.insert(peer, tel);
                                                }
                                            }
                                        }
                                    },

                                    SwarmEvent::Behaviour(SynapseBehaviorEvent::ReqRes(request_response::Event::Message { peer, message })) => {
                                        if let request_response::Message::Response { request_id, response } = message {
                                            if let Some(hash) = req_to_hash.remove(&request_id) {
                                                if let SwarmResponse::DataPayload(bytes) = response {
                                                    if let Some(tx) = hash_to_tx.remove(&hash) {
                                                        let _ = tx.send(Some(bytes));
                                                    }
                                                }
                                            }
                                        } else if let request_response::Message::Request { request: SwarmRequest::SubmitResult(json_payload), channel, .. } = message {
                                            let _ = p2p_node.swarm.behaviour_mut().req_res.send_response(channel, SwarmResponse::Ack);

                                            let jobs_clone = jobs_c.clone();
                                            let health_clone = health_c.clone();
                                            let contract_states_clone = contract_states_c.clone();
                                            let tx_clone = tx.clone();
                                            // PHASE 14 & THE TOKIO ASYNC LAW:
                                            // Offloading strict lock operations to a dedicated spawned task prevents blocking the Libp2p loop
                                            tokio::spawn(async move {
                                                if let Ok(res_data) = serde_json::from_str::<ShardResult>(&json_payload) {
                                                    health_clone.remove(&peer);

                                                    let job_arc_opt = jobs_clone.get(&res_data.job_id).map(|ref_val| ref_val.value().clone());

                                                    if let Some(job_arc) = job_arc_opt {
                                                        let mut guard = job_arc.lock().await;

                                                        if guard.verified_results.contains_key(&res_data.shard_index) { return; }

                                                        let is_consensus_ready = {
                                                            let shard_results = guard.raw_results.entry(res_data.shard_index).or_insert_with(HashMap::new);
                                                            shard_results.insert(peer, (res_data.result, res_data.result_hash.clone(), res_data.state_delta.clone(), res_data.execution_timestamp));

                                                            let display_hash = if res_data.result_hash.len() >= 8 { &res_data.result_hash[..8] } else { &res_data.result_hash };
                                                            println!("📩 Received Result Hash: [{}] from {}", display_hash, peer);

                                                            shard_results.len() >= guard.redundancy
                                                        };

                                                        if is_consensus_ready {
                                                            if let Some(shard_results) = guard.raw_results.remove(&res_data.shard_index) {
                                                                // BFT Deduplication based strictly on matching results & hashes
                                                                let mut unique_hashes: HashMap<String, (i32, String, BTreeMap<String, String>, u64)> = HashMap::new();
                                                                for (r, h, d, ts) in shard_results.values() {
                                                                    unique_hashes.insert(h.clone(), (*r, h.clone(), d.clone(), *ts));
                                                                }

                                                                if unique_hashes.len() == 1 {
                                                                    let (_, (final_result, final_hash, final_delta, final_ts)) = unique_hashes.into_iter().next().unwrap();

                                                                    let final_display = if final_hash.len() >= 8 { &final_hash[..8] } else { &final_hash };
                                                                    println!("✅ HASH CONSENSUS REACHED: Shard {} state verified [{}]", res_data.shard_index, final_display);

                                                                    guard.verified_results.insert(res_data.shard_index, (final_result, final_hash.clone(), final_delta, final_ts));

                                                                    // PHASE 14: MULTI-SHARD HYBRID CRDT MERGE
                                                                    if guard.verified_results.len() == guard.expected_shards {
                                                                        println!("🌐 ALL SHARDS VERIFIED! Initiating Phase 14 Multi-Shard State Merge...");

                                                                        let mut master_state: BTreeMap<String, String> = BTreeMap::new();
                                                                        let mut key_timestamps: HashMap<String, u64> = HashMap::new();

                                                                        for (_, _, delta, ts) in guard.verified_results.values() {
                                                                            for (k, v) in delta {
                                                                                if let Some(existing_v) = master_state.get(k) {
                                                                                    // Commutative Addition for numerics
                                                                                    if let (Ok(ex_i64), Ok(new_i64)) = (existing_v.parse::<i64>(), v.parse::<i64>()) {
                                                                                        master_state.insert(k.clone(), (ex_i64 + new_i64).to_string());
                                                                                    }
                                                                                    else if let (Ok(ex_f64), Ok(new_f64)) = (existing_v.parse::<f64>(), v.parse::<f64>()) {
                                                                                        master_state.insert(k.clone(), (ex_f64 + new_f64).to_string());
                                                                                    }
                                                                                    // Last-Write-Wins for strings/objects
                                                                                    else {
                                                                                        let existing_ts = key_timestamps.get(k).copied().unwrap_or(0);
                                                                                        if *ts > existing_ts {
                                                                                            master_state.insert(k.clone(), v.clone());
                                                                                            key_timestamps.insert(k.clone(), *ts);
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    master_state.insert(k.clone(), v.clone());
                                                                                    key_timestamps.insert(k.clone(), *ts);
                                                                                }
                                                                            }
                                                                        }

                                                                        // Generate Deterministic Canonical Zero-Whitespace Hash
                                                                        let merged_json = serde_json::to_string(&master_state).unwrap();
                                                                        let mut hasher = Sha256::new();
                                                                        hasher.update(merged_json.as_bytes());
                                                                        let final_master_hash = hex::encode(hasher.finalize());

                                                                        println!("🚀 PHASE 14 MERGE COMPLETE: Master State Hash -> [{}]", final_master_hash);
                                                                        guard.master_state_hash = Some(final_master_hash.clone());

                                                                        let is_stateful = !guard.wasm_image.is_empty() && guard.wasm_image != b"NONE";
                                                                        if is_stateful {
                                                                            let mut img_hasher = Sha256::new();
                                                                            img_hasher.update(&guard.wasm_image);
                                                                            let contract_key = hex::encode(img_hasher.finalize());

                                                                            contract_states_clone.insert(contract_key.clone(), final_master_hash.clone());
                                                                            let sync_msg = format!("SYNC_STATE:{{\"k\":\"{}\",\"v\":\"{}\"}}", contract_key, final_master_hash);

                                                                            if let Err(e) = tx_clone.try_send(NodeCommand::GatewaySync(sync_msg)) {
                                                                                eprintln!("⚠️ BACKPRESSURE ALARM: Failed to send GatewaySync: {}", e);
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    println!("🚨 HASH COLLISION DETECTED! State mutation mismatch on Shard {}.", res_data.shard_index);
                                                                    for p in shard_results.keys() {
                                                                        if let Err(e) = tx_clone.try_send(NodeCommand::Disconnect(*p)) {
                                                                            eprintln!("⚠️ BACKPRESSURE ALARM: Failed to send Disconnect: {}", e);
                                                                        }
                                                                    }
                                                                    guard.assignments.remove(&res_data.shard_index);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            });
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
        .route("/api/v1/data/:hash", get(fetch_data))
        .layer(axum::extract::DefaultBodyLimit::max(50_000_000))
        .with_state(shared_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    println!("Gateway Active: http://localhost:{}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn submit_job(
    State(state): State<Arc<AppState>>,
    mut multipart: axum::extract::Multipart,
) -> (StatusCode, Json<JobSubmitResponse>) {
    let task_id = Uuid::new_v4();
    let mut wasm_bytes = Vec::new();
    let mut dataset = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "wasm" {
            if let Ok(data) = field.bytes().await {
                wasm_bytes = data.to_vec();
            }
        } else if name == "metadata" {
            if let Ok(data) = field.bytes().await {
                let text = String::from_utf8_lossy(&data);
                if let Ok(payload) = serde_json::from_str::<ShardedDeployRequest>(&text) {
                    dataset = payload.dataset;
                }
            }
        }
    }

    let job_state = JobState {
        expected_shards: 0,
        redundancy: 2,
        raw_results: HashMap::new(),
        verified_results: HashMap::new(),
        master_state_hash: None,
        created_at: Instant::now(),
        assignments: HashMap::new(),
        shards_data: HashMap::new(),
        unassigned_dataset: Some(dataset.clone()),
        wasm_image: wasm_bytes,
    };

    state.jobs.insert(task_id, Arc::new(Mutex::new(job_state)));

    println!(
        "📥 Job Queued: {} with {} Complex Items",
        task_id,
        dataset.len()
    );

    (
        StatusCode::ACCEPTED,
        Json(JobSubmitResponse {
            job_id: task_id.to_string(),
            status: "pending_scheduler".to_string(),
        }),
    )
}

async fn get_job_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<JobStatusResponse>) {
    let job_arc_opt = state.jobs.get(&id).map(|r| r.value().clone());

    if let Some(job_arc) = job_arc_opt {
        let guard = job_arc.lock().await;

        if guard.unassigned_dataset.is_some() {
            return (
                StatusCode::ACCEPTED,
                Json(JobStatusResponse {
                    status: "awaiting_telemetry".to_string(),
                    total_sum: 0,
                    breakdown: vec![],
                    hashes: vec![],
                    missing_shards: vec![],
                    master_state_hash: None,
                }),
            );
        }

        let is_complete = guard.verified_results.len() >= guard.expected_shards;
        let sum: i32 = guard.verified_results.values().map(|v| v.0).sum();

        let breakdown: Vec<(u32, i32)> = guard
            .verified_results
            .iter()
            .map(|(k, v)| (*k, v.0))
            .collect();
        let hashes: Vec<(u32, String)> = guard
            .verified_results
            .iter()
            .map(|(k, v)| (*k, v.1.clone()))
            .collect();

        let received_indices: HashSet<u32> = guard.verified_results.keys().copied().collect();
        let missing_indices: Vec<u32> = (0..guard.expected_shards as u32)
            .filter(|i| !received_indices.contains(i))
            .collect();

        if is_complete {
            (
                StatusCode::OK,
                Json(JobStatusResponse {
                    status: "completed".to_string(),
                    total_sum: sum,
                    breakdown,
                    hashes,
                    missing_shards: vec![],
                    master_state_hash: guard.master_state_hash.clone(),
                }),
            )
        } else {
            (
                StatusCode::PARTIAL_CONTENT,
                Json(JobStatusResponse {
                    status: "partial".to_string(),
                    total_sum: sum,
                    breakdown,
                    hashes,
                    missing_shards: missing_indices,
                    master_state_hash: guard.master_state_hash.clone(),
                }),
            )
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(JobStatusResponse {
                status: "not_found".to_string(),
                total_sum: 0,
                breakdown: vec![],
                hashes: vec![],
                missing_shards: vec![],
                master_state_hash: None,
            }),
        )
    }
}

async fn dashboard(State(_state): State<Arc<AppState>>) -> Html<String> {
    Html(
        r#"
        <div style="font-family: sans-serif; padding: 2rem;">
            <h1>🐝 Swarm Runtime Gateway</h1>
            <p><strong>Version:</strong> v0.26.0 (Enterprise Federation)</p>
        </div>
    "#
        .to_string(),
    )
}

async fn fetch_data(
    State(state): State<Arc<AppState>>,
    Path(hash): Path<String>,
) -> (StatusCode, Vec<u8>) {
    let (tx, rx) = tokio::sync::oneshot::channel();

    if let Err(e) = state
        .node_tx
        .try_send(NodeCommand::FetchFile(hash.clone(), tx))
    {
        eprintln!("⚠️ BACKPRESSURE ALARM: Failed to send FetchFile: {}", e);
    }

    match tokio::time::timeout(std::time::Duration::from_secs(15), rx).await {
        Ok(Ok(Some(bytes))) => (StatusCode::OK, bytes),
        _ => (StatusCode::NOT_FOUND, b"File not found on network".to_vec()),
    }
}
