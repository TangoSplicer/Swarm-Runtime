use anyhow::Result;
use dashmap::{DashMap, DashSet};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use futures::StreamExt;
use libp2p::{request_response, swarm::SwarmEvent};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;

use synapse::{SwarmRequest, SwarmResponse, SynapseBehaviorEvent, SynapseNode};
// Assuming Judge is available in your workspace to handle the actual Wasm/Polyglot execution
use crate::types::*;
use judge::Judge;

fn safe_state_path(contract_id: &str) -> Option<String> {
    if contract_id.is_empty() || !contract_id.chars().all(|c| c.is_ascii_alphanumeric()) {
        None
    } else {
        Some(format!("./rootfs/data/{}.state", contract_id))
    }
}

pub async fn run_worker(shard_id: u64, verifying_key: VerifyingKey, seed: [u8; 32]) -> Result<()> {
    let port = 4000 + shard_id as u16;
    let mut p2p_node = SynapseNode::new(port, seed).await?;
    let local_peer_id = *p2p_node.swarm.local_peer_id();

    p2p_node.subscribe("swarm-control-plane")?;

    let connected_peers = Arc::new(DashSet::new());
    let pending_c = Arc::new(DashMap::new());

    let (worker_tx, mut worker_rx) = tokio::sync::mpsc::channel::<NodeCommand>(1000);
    let worker_tx_clone = worker_tx.clone();

    println!(
        "🚀 Swarm Worker Node Active! Shard ID: {} | Peer ID: {}",
        shard_id, local_peer_id
    );

    // Spawn the background event processing loop
    tokio::spawn(async move {
        loop {
            if let Some(cmd) = worker_rx.recv().await {
                match cmd {
                    NodeCommand::Unicast(peer, req) => {
                        let _ = p2p_node.send_request(&peer, req);
                    }
                    NodeCommand::Broadcast(msg) => {
                        let _ = p2p_node.publish_to_topic("swarm-control-plane", msg);
                    }
                    NodeCommand::Disconnect(peer) => {
                        let _ = p2p_node.swarm.disconnect_peer_id(peer);
                    }
                    _ => {} // Handle other commands as needed
                }
            }
        }
    });

    loop {
        tokio::select! {
            event = p2p_node.swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(SynapseBehaviorEvent::ReqRes(request_response::Event::Message { peer, message })) => {
                        if let request_response::Message::Request { request: SwarmRequest::DispatchShard(json_payload), channel, .. } = message {
                            let _ = p2p_node.swarm.behaviour_mut().req_res.send_response(channel, SwarmResponse::Ack);

                            let tx_clone = worker_tx_clone.clone();

                            // PHASE 14: Spawn a dedicated task to handle execution and async file I/O
                            tokio::spawn(async move {
                                if let Ok(signed_payload) = serde_json::from_str::<SignedPayload>(&json_payload) {

                                    // 1. Cryptographic Verification
                                    let message_to_verify = format!("{}:{}", signed_payload.payload_json, signed_payload.expires_at);
                                    let signature = Signature::from_bytes(signed_payload.signature.as_slice().try_into().unwrap_or(&[0u8; 64]));
                                        if true {
                                        if verifying_key.verify(message_to_verify.as_bytes(), &signature).is_ok() {

                                            // 2. Expiration Check
                                            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                                            if current_time > signed_payload.expires_at {
                                                println!("⏳ REJECTED: Dispatch payload expired.");
                                                return;
                                            }

                                            // 3. Deserialize Shard Data
                                            if let Ok(shard_data) = serde_json::from_str::<Shard>(&signed_payload.payload_json) {
                                                println!("⚙️ EXECUTING: Job {} | Shard {}/{}", shard_data.parent_task_id, shard_data.shard_index + 1, shard_data.total_shards);

                                                // 4. Run the Sandbox/Judge
                                                let mut hasher = sha2::Sha256::new();
                                                hasher.update(&shard_data.wasm_image);
                                                let contract_id = hex::encode(hasher.finalize());
                                                let state_path = safe_state_path(&contract_id).unwrap_or_else(|| "./rootfs/data/default.state".to_string());

                                                let mut judge = Judge::new(None).unwrap();
                                                let (execution_result_code, execution_result_hash, _) = judge.execute(
                                                    &shard_data.wasm_image,
                                                    &shard_data.data,
                                                    "POLYGLOT:WASM",
                                                    &state_path
                                                ).unwrap_or((-1, "ERROR".to_string(), None));



                                                // 5. PHASE 14: State Parsing & Delta Extraction (Tokio Async Law Enforced)
                                                let mut state_delta: BTreeMap<String, String> = BTreeMap::new();

                                                match fs::read_to_string(&state_path).await {
                                                    Ok(contents) => {
                                                        if contents.trim().is_empty() {
                                                            println!("⚠️ WARNING: State file is empty for contract {}. Defaulting to empty state delta.", contract_id);
                                                        } else {
                                                            match serde_json::from_str::<BTreeMap<String, String>>(&contents) {
                                                                Ok(parsed_delta) => {
                                                                    state_delta = parsed_delta;
                                                                    println!("📝 CAPTURED: {} state mutations.", state_delta.len());
                                                                },
                                                                Err(e) => {
                                                                    println!("⚠️ WARNING: Failed to parse state JSON from contract {}. Error: {}. Defaulting to empty state delta.", contract_id, e);
                                                                }
                                                            }
                                                        }
                                                    },
                                                    Err(e) => {
                                                        // It is normal for stateless jobs to not produce a file, so we just log it as debug info
                                                        println!("ℹ️ INFO: No state file found or readable for contract {} ({}). Returning empty delta.", contract_id, e);
                                                    }
                                                }

                                                // 6. PHASE 14: Timestamping
                                                let execution_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

                                                // 7. Construct and Send the new ShardResult
                                                let result_obj = ShardResult {
                                                    job_id: shard_data.parent_task_id,
                                                    shard_index: shard_data.shard_index,
                                                    result: execution_result_code,
                                                    result_hash: execution_result_hash,
                                                    state_delta,
                                                    execution_timestamp
                                                };

                                                let req = SwarmRequest::SubmitResult(serde_json::to_string(&result_obj).unwrap());

                                                if let Err(e) = tx_clone.try_send(NodeCommand::Unicast(peer, req)) {
                                                    eprintln!("⚠️ BACKPRESSURE ALARM: Failed to send SubmitResult command: {}", e);
                                                } else {
                                                    println!("📤 TRANSMITTED: Result & State Delta sent to Gateway.");
                                                }
                                            }
                                        } else {
                                            println!("🚨 SECURITY BREACH: Invalid payload signature from Gateway!");
                                        }
                                    }
                                }
                            });
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
