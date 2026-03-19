use anyhow::Result;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use libp2p::{swarm::SwarmEvent, mdns, request_response, kad};
use futures::StreamExt;
use std::sync::Arc;
use dashmap::{DashMap, DashSet};
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use sysinfo::System;
use std::fs;
use sha2::{Sha256, Digest};
use tokio::sync::Mutex;
use std::collections::HashMap;

use synapse::{SynapseNode, SynapseBehaviorEvent, SwarmRequest, SwarmResponse};
use judge::Judge;
use crate::types::*;

fn safe_state_path(contract_id: &str) -> Option<String> {
    if contract_id.is_empty() || !contract_id.chars().all(|c| c.is_ascii_alphanumeric()) {
        None
    } else {
        Some(format!("./rootfs/data/{}.state", contract_id))
    }
}

pub async fn run_worker(shard_id: u64, verifying_key: VerifyingKey, seed: [u8; 32]) -> Result<()>  {
    let port = 4000 + shard_id as u16;
    let mut p2p_node = SynapseNode::new(port, seed).await?;
    let local_peer_id = *p2p_node.swarm.local_peer_id();
    p2p_node.subscribe("swarm-control-plane")?;

    let bootstrap_nodes = vec![
        "/ip4/145.241.192.79/tcp/4000/p2p/12D3KooWR6zCohbghjfRyUriggGyq6HKWJxapyziq2icYgXGeEiK",
    ];

    let mut dialed_any = false;
    for node_str in bootstrap_nodes {
        if let Ok(multiaddr) = node_str.parse::<libp2p::Multiaddr>() {
            if p2p_node.swarm.dial(multiaddr.clone()).is_ok() {
                println!("🚀 Dialing Federated Gateway: {}", node_str);
                dialed_any = true;
            }
        }
    }
    if !dialed_any {
        println!("⚠️ WARNING: Could not dial any federated Gateways. Operating in local-only mDNS mode.");
    }
    println!("=== Worker Live (Shard {}) on Port {} ===", shard_id, port);

    let pending_dials = Arc::new(DashMap::<libp2p::PeerId, Instant>::new());
    let pending_c = pending_dials.clone();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<NodeCommand>(1000);

    let state_locks = Arc::new(DashMap::<String, Arc<Mutex<()>>>::new());
    let seen_jobs = Arc::new(DashSet::<String>::new());
    
    let connected_peers = Arc::new(DashSet::<libp2p::PeerId>::new());
    let (_internal_fetch_tx, mut internal_fetch_rx) = tokio::sync::mpsc::channel::<(String, libp2p::PeerId, tokio::sync::oneshot::Sender<Vec<u8>>)>(100);
    let mut fetch_callbacks = HashMap::<libp2p::request_response::OutboundRequestId, tokio::sync::oneshot::Sender<Vec<u8>>>::new();

    let worker_tx_tel = tx.clone();
    let my_peer_id = local_peer_id.to_string();
    let connected_peers_tel = connected_peers.clone();

    tokio::spawn(async move {
        let mut sys = System::new_all();
        loop {
            // PHASE 13: Algorithmic Scaling - Dynamic Square-Root Telemetry Backoff
            let active_peers = connected_peers_tel.len();
            let backoff_seconds = std::cmp::max(10, (active_peers as f64).sqrt() as u64 * 5);
            tokio::time::sleep(Duration::from_secs(backoff_seconds)).await;
            
            sys.refresh_cpu_usage();
            sys.refresh_memory();

            let cpu_usage = sys.global_cpu_usage();
            let free_ram_mb = sys.available_memory() / 1024 / 1024;

            let tel = Telemetry { peer_id: my_peer_id.clone(), cpu_usage, free_ram_mb };
            if let Ok(json) = serde_json::to_string(&tel) {
                if let Err(e) = worker_tx_tel.try_send(NodeCommand::Broadcast(format!("TEL:{}", json))) {
                    eprintln!("⚠️ BACKPRESSURE ALARM: Failed to send Telemetry: {}", e);
                }
            }
        }
    });

    loop {
        tokio::select! {
            Some((hash, target_peer, reply_tx)) = internal_fetch_rx.recv() => {
                let req_id = p2p_node.send_request(&target_peer, SwarmRequest::FetchData(hash));
                fetch_callbacks.insert(req_id, reply_tx);
            },
            Some(cmd) = rx.recv() => {
                match cmd {
                    NodeCommand::Unicast(peer, req) => { let _ = p2p_node.send_request(&peer, req); },
                    NodeCommand::Broadcast(msg) => { let _ = p2p_node.publish_to_topic("swarm-control-plane", msg); },
                    NodeCommand::GatewaySync(_) => {},
                    NodeCommand::FetchFile(_, _) => {}, 
                    NodeCommand::Disconnect(peer) => {
                        println!("⛔ BANNING PEER: {}", peer);
                        let _ = p2p_node.swarm.disconnect_peer_id(peer);
                    },
                    NodeCommand::PinFile(hash) => {
                        let record = kad::Record {
                            key: kad::RecordKey::new(&hash),
                            value: local_peer_id.to_bytes(),
                            publisher: None,
                            expires: None,
                        };
                        if let Err(e) = p2p_node.swarm.behaviour_mut().kademlia.put_record(record, kad::Quorum::One) {
                            println!("⚠️ DHT Pin Error: {:?}", e);
                        } else {
                            println!("🌐 DHT: Successfully announced file [{}] to the Mesh!", &hash[..8]);
                        }
                    }
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
                        if let request_response::Message::Response { request_id, response } = message {
                            if let Some(tx) = fetch_callbacks.remove(&request_id) {
                                if let SwarmResponse::DataPayload(bytes) = response {
                                    let _ = tx.send(bytes);
                                }
                            }
                        } else if let request_response::Message::Request { request: SwarmRequest::FetchData(hash), channel, .. } = message {
                                println!("📥 Received FetchData request for Hash: [{}]", &hash[..8]);
                                let mut file_bytes = Vec::new();

                                if let Ok(entries) = std::fs::read_dir("./rootfs/data") {
                                    for entry in entries.flatten() {
                                        if let Ok(content) = std::fs::read(entry.path()) {
                                            let mut hasher = Sha256::new();
                                            hasher.update(&content);
                                            if hex::encode(hasher.finalize()) == hash {
                                                file_bytes = content;
                                                break;
                                            }
                                        }
                                    }
                                }

                                if file_bytes.is_empty() {
                                    println!("❌ File not found in VMFS for Hash: [{}]", &hash[..8]);
                                    p2p_node.send_response(channel, SwarmResponse::Error("File not found in VMFS".to_string()));
                                } else {
                                    println!("📤 Streaming {} bytes back to Gateway...", file_bytes.len());
                                    p2p_node.send_response(channel, SwarmResponse::DataPayload(file_bytes));
                                }
                            } else if let request_response::Message::Request { request: SwarmRequest::DispatchShard(json_payload), channel, .. } = message {
                            p2p_node.send_response(channel, SwarmResponse::Ack);

                            if let Ok(envelope) = serde_json::from_str::<SignedPayload>(&json_payload) {
                                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                                if now > envelope.expires_at {
                                    println!("🚨 REPLAY ATTACK: Payload expired.");
                                    if let Err(e) = tx.try_send(NodeCommand::Disconnect(peer)) {
                                        eprintln!("⚠️ BACKPRESSURE ALARM: Failed to send Disconnect command: {}", e);
                                    }
                                    continue;
                                }

                                let message_to_verify = format!("{}:{}", envelope.payload_json, envelope.expires_at);
                                if let Ok(sig_bytes) = envelope.signature.try_into() {
                                    let signature = Signature::from_bytes(&sig_bytes);
                                    if verifying_key.verify(message_to_verify.as_bytes(), &signature).is_err() {
                                        println!("🚨 SECURITY ALERT: Cryptographic signature verification failed. Dropping payload.");
                                        continue;
                                    }
                                } else {
                                    println!("🚨 SECURITY ALERT: Malformed signature bytes. Dropping payload.");
                                    continue;
                                }

                                if let Ok(shard_data) = serde_json::from_str::<Shard>(&envelope.payload_json) {
                                    
                                    let job_id_str = shard_data.parent_task_id.to_string();
                                    if seen_jobs.contains(&job_id_str) {
                                        println!("⏭️ Skipping Shard {} (Already computing a shard for Job [{}...])", shard_data.shard_index, &job_id_str[..8]);
                                        continue; 
                                    }
                                    seen_jobs.insert(job_id_str);

                                    println!("🔒 Verified & Claiming Shard {}", shard_data.shard_index);

                                    let worker_tx = tx.clone();
                                    let gateway_id = peer;
                                    let locks_clone = state_locks.clone();
                                    
                                    tokio::spawn(async move {
                                        let polyglot_id = if shard_data.wasm_image.starts_with(b"POLYGLOT:") { shard_data.wasm_image.clone() } else { b"NONE".to_vec() };
                                        let wasm_result = match String::from_utf8_lossy(&polyglot_id).as_ref() {
                                            "POLYGLOT:PYTHON" => Ok(std::fs::read("python.wasm").unwrap_or_default()),
                                            "POLYGLOT:JS" => Ok(std::fs::read("qjs.wasm").unwrap_or_default()),
                                            "POLYGLOT:LUA" => Ok(std::fs::read("lua.wasm").unwrap_or_default()),
                                            "POLYGLOT:RUBY" => Ok(std::fs::read("ruby.wasm").unwrap_or_default()),
                                            "POLYGLOT:PHP" => Ok(std::fs::read("php.wasm").unwrap_or_default()),
                                            "POLYGLOT:SQLITE" => Ok(std::fs::read("sqlite.wasm").unwrap_or_default()),
                                            _ => general_purpose::STANDARD.decode(&shard_data.wasm_image),
                                        };
                                        
                                        if let Ok(wasm) = wasm_result {
                                            let mut contract_hasher = Sha256::new();
                                            contract_hasher.update(&wasm);
                                            let contract_id = hex::encode(contract_hasher.finalize());
                                            
                                            let state_path = match safe_state_path(&contract_id) {
                                                Some(path) => path,
                                                None => {
                                                    println!("🚨 SECURITY ALERT: Invalid contract ID. Path traversal aborted.");
                                                    return;
                                                }
                                            };
                                            
                                            let contract_lock = locks_clone.entry(contract_id.clone()).or_insert_with(|| Arc::new(Mutex::new(()))).value().clone();
                                            let _guard = contract_lock.lock().await;

                                            let mut judge = Judge::new(None).unwrap();

                                            match judge.execute(&wasm, &shard_data.data, String::from_utf8_lossy(&polyglot_id).as_ref(), &state_path) {
                                                Ok((res, hash, new_state_opt)) => {
                                                    let sandbox_dir = "./rootfs/data";
                                                    let mut actual_state_hash = hash.clone();

                                                    if let Some(state_bytes) = new_state_opt {
                                                        let mut state_hasher = Sha256::new();
                                                        state_hasher.update(&state_bytes);
                                                        actual_state_hash = hex::encode(state_hasher.finalize());
                                                        
                                                        if let Err(e) = fs::write(&state_path, state_bytes) {
                                                            println!("⚠️ Failed to save state to VMFS: {}", e);
                                                        } else {
                                                            println!("💾 VMFS: Saved new state snapshot for Contract [{}]", &contract_id[..8]);
                                                        }
                                                    }

                                                    if let Ok(entries) = fs::read_dir(sandbox_dir) {
                                                        for entry in entries.flatten() {
                                                            if let Ok(meta) = entry.metadata() {
                                                                if meta.is_file() {
                                                                    let path = entry.path();
                                                                    if let Ok(bytes) = fs::read(&path) {
                                                                        let mut hasher = Sha256::new();
                                                                        hasher.update(&bytes);
                                                                        let file_hash = format!("{:x}", hasher.finalize());
                                                                        
                                                                        if let Err(e) = worker_tx.try_send(NodeCommand::PinFile(file_hash.clone())) {
                                                                            eprintln!("⚠️ BACKPRESSURE ALARM: Failed to send PinFile command: {}", e);
                                                                        }
                                                                        let _ = file_hash; 
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }

                                                    let short_hash = if actual_state_hash.len() >= 8 { &actual_state_hash[..8] } else { &actual_state_hash };
                                                    println!("✅ Result: {} | True State Hash: [{}]", res, short_hash);

                                                    let result_obj = ShardResult { 
                                                        job_id: shard_data.parent_task_id, 
                                                        shard_index: shard_data.shard_index, 
                                                        result: res, 
                                                        result_hash: actual_state_hash
                                                    };
                                                    let req = SwarmRequest::SubmitResult(serde_json::to_string(&result_obj).unwrap());
                                                    if let Err(e) = worker_tx.try_send(NodeCommand::Unicast(gateway_id, req)) {
                                                        eprintln!("⚠️ BACKPRESSURE ALARM: Failed to send SubmitResult command: {}", e);
                                                    }
                                                },
                                                Err(e) => {
                                                    println!("❌ Sandbox Error: {}", e);
                                                    let result_obj = ShardResult { job_id: shard_data.parent_task_id, shard_index: shard_data.shard_index, result: -1, result_hash: "ERROR".to_string() };
                                                    let req = SwarmRequest::SubmitResult(serde_json::to_string(&result_obj).unwrap());
                                                    if let Err(e) = worker_tx.try_send(NodeCommand::Unicast(gateway_id, req)) {
                                                        eprintln!("⚠️ BACKPRESSURE ALARM: Failed to send SubmitResult error command: {}", e);
                                                    }
                                                }
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    },
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        println!("📡 CONNECTED: {}", peer_id);
                        connected_peers.insert(peer_id);
                        pending_c.remove(&peer_id);
                    },
                    SwarmEvent::ConnectionClosed { peer_id, .. } => {
                        println!("📉 DISCONNECTED: {}", peer_id);
                        connected_peers.remove(&peer_id);
                    },
                    _ => {}
                }
            }
        }
    }
}
