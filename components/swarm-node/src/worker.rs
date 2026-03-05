use anyhow::Result;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use libp2p::{swarm::SwarmEvent, mdns, request_response, kad};
use futures::StreamExt;
use std::sync::Arc;
use dashmap::DashMap;
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use sysinfo::System;
use std::fs;
use sha2::{Sha256, Digest};
use tokio::sync::Mutex; // NEW: Bring in Tokio's async Mutex

use synapse::{SynapseNode, SynapseBehaviorEvent, SwarmRequest, SwarmResponse};
use judge::Judge;
use crate::types::*;

pub async fn run_worker(shard_id: u64, verifying_key: VerifyingKey) -> Result<()> {
    let port = 4000 + shard_id as u16;
    let mut p2p_node = SynapseNode::new(port).await?;
    let local_peer_id = *p2p_node.swarm.local_peer_id();
    p2p_node.subscribe("swarm-control-plane")?;
    println!("=== Worker Live (Shard {}) on Port {} ===", shard_id, port);

    let pending_dials = Arc::new(DashMap::<libp2p::PeerId, Instant>::new());
    let pending_c = pending_dials.clone();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<NodeCommand>();

    // NEW: Global Lock Manager for Stateful Smart Contracts
    let state_locks = Arc::new(DashMap::<String, Arc<Mutex<()>>>::new());

    let worker_tx_tel = tx.clone();
    let my_peer_id = local_peer_id.to_string();

    tokio::spawn(async move {
        let mut sys = System::new_all();
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            sys.refresh_cpu_usage();
            sys.refresh_memory();

            let cpu_usage = sys.global_cpu_usage();
            let free_ram_mb = sys.available_memory() / 1024 / 1024;

            let tel = Telemetry { peer_id: my_peer_id.clone(), cpu_usage, free_ram_mb };
            if let Ok(json) = serde_json::to_string(&tel) {
                let _ = worker_tx_tel.send(NodeCommand::Broadcast(format!("TEL:{}", json)));
            }
        }
    });

    loop {
        tokio::select! {
            Some(cmd) = rx.recv() => {
                match cmd {
                    NodeCommand::Unicast(peer, req) => { let _ = p2p_node.send_request(&peer, req); },
                    NodeCommand::Broadcast(msg) => { let _ = p2p_node.publish_to_topic("swarm-control-plane", msg); },
                    NodeCommand::FetchFile(_, _) => {}, // Ignored: Gateway-only command
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
                        if let request_response::Message::Request { request: SwarmRequest::FetchData(hash), channel, .. } = message {
                                println!("📥 Received FetchData request for Hash: [{}]", hash);
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
                                    println!("❌ File not found in VMFS for Hash: [{}]", hash);
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
                                    let _ = tx.send(NodeCommand::Disconnect(peer));
                                    continue;
                                }

                                let message_to_verify = format!("{}:{}", envelope.payload_json, envelope.expires_at);
                                if let Ok(sig_bytes) = envelope.signature.try_into() {
                                    let signature = Signature::from_bytes(&sig_bytes);
                                    if verifying_key.verify(message_to_verify.as_bytes(), &signature).is_ok() {
                                        if let Ok(shard_data) = serde_json::from_str::<Shard>(&envelope.payload_json) {
                                            println!("🔒 Verified & Claiming Shard {}", shard_data.shard_index);

                                            let worker_tx = tx.clone();
                                            let gateway_id = peer;
                                            // Pass a clone of the lock manager into the Tokio thread
                                            let locks_clone = state_locks.clone();

                                            tokio::spawn(async move {
                                                let polyglot_id = if shard_data.wasm_image.starts_with("POLYGLOT:") { shard_data.wasm_image.clone() } else { "NONE".to_string() };
                                                let wasm_result = match polyglot_id.as_str() {
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
                                                    let state_path = format!("./rootfs/data/{}.state", contract_id);
                                                    
                                                    // --- ASYNC MUTEX LOCK ---
                                                    // This ensures if a worker claims Shard 0 and Shard 1 of the same contract,
                                                    // they queue up and execute sequentially, preserving state integrity!
                                                    let contract_lock = locks_clone.entry(contract_id.clone()).or_insert_with(|| Arc::new(Mutex::new(()))).value().clone();
                                                    let _guard = contract_lock.lock().await;

                                                    let previous_state = fs::read(&state_path).ok();
                                                    if previous_state.is_some() {
                                                        println!("🔄 VMFS: Reloading persistent state for Contract [{}]", &contract_id[..8]);
                                                    }

                                                    let mut judge = Judge::new(None).unwrap();

                                                    match judge.execute(&wasm, &shard_data.data, &polyglot_id, previous_state.as_deref()) {
                                                        Ok((res, mut hash, new_state_opt)) => {
                                                            let sandbox_dir = "./rootfs/data";

                                                            if let Some(state_bytes) = new_state_opt {
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
                                                                                
                                                                                let _ = worker_tx.send(NodeCommand::PinFile(file_hash.clone()));
                                                                                hash = file_hash;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }

                                                            let short_hash = if hash.len() >= 8 { &hash[..8] } else { &hash };
                                                            println!("✅ Result: {} | Output Hash: [{}]", res, short_hash);

                                                            let result_obj = ShardResult { job_id: shard_data.parent_task_id, shard_index: shard_data.shard_index, result: res, result_hash: hash };
                                                            let req = SwarmRequest::SubmitResult(serde_json::to_string(&result_obj).unwrap());
                                                            let _ = worker_tx.send(NodeCommand::Unicast(gateway_id, req));
                                                        },
                                                        Err(e) => {
                                                            println!("❌ Sandbox Error: {}", e);
                                                            let result_obj = ShardResult { job_id: shard_data.parent_task_id, shard_index: shard_data.shard_index, result: -1, result_hash: "ERROR".to_string() };
                                                            let req = SwarmRequest::SubmitResult(serde_json::to_string(&result_obj).unwrap());
                                                            let _ = worker_tx.send(NodeCommand::Unicast(gateway_id, req));
                                                        }
                                                    }
                                                    // _guard goes out of scope here, releasing the lock for the next shard!
                                                }
                                            });
                                        }
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
