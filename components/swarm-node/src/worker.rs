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
                    NodeCommand::Disconnect(peer) => { 
                        println!("⛔ BANNING PEER: {}", peer);
                        let _ = p2p_node.swarm.disconnect_peer_id(peer); 
                    },
                    // --- KADEMLIA DHT PUBLISHER ---
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
                        if let request_response::Message::Request { request: SwarmRequest::DispatchShard(json_payload), channel, .. } = message {
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
                                            
                                            tokio::spawn(async move {
                                                if let Ok(wasm) = general_purpose::STANDARD.decode(&shard_data.wasm_image) {
                                                    let mut judge = Judge::new(None).unwrap();
                                                    
                                                    match judge.execute(&wasm, &shard_data.data) {
                                                        Ok((res, mut hash)) => {
                                                            let sandbox_dir = "./swarm_data";
                                                            if let Ok(entries) = fs::read_dir(sandbox_dir) {
                                                                for entry in entries.flatten() {
                                                                    if let Ok(meta) = entry.metadata() {
                                                                        if meta.is_file() {
                                                                            let path = entry.path();
                                                                            if let Ok(bytes) = fs::read(&path) {
                                                                                let mut hasher = Sha256::new();
                                                                                hasher.update(&bytes);
                                                                                let file_hash = format!("{:x}", hasher.finalize());
                                                                                
                                                                                println!("📁 VMFS: Found new file {} (Hash: [{}])", path.display(), &file_hash[..8]);
                                                                                
                                                                                // 1. Tell libp2p to announce this to the Kademlia DHT
                                                                                let _ = worker_tx.send(NodeCommand::PinFile(file_hash.clone()));
                                                                                
                                                                                // 2. Lock the file hash in as our execution state
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
