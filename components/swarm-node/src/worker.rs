#![allow(unused)]
#![allow(clippy::collapsible_match)]
use anyhow::Result;
use dashmap::DashSet;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use futures::StreamExt;
use libp2p::{request_response, swarm::SwarmEvent};
use sha2::Digest;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use synapse::{SwarmRequest, SwarmResponse, SynapseBehaviorEvent, SynapseNode};
// Assuming Judge is available in your workspace to handle the actual Wasm/Polyglot execution
use crate::types::*;
use judge::Judge;

fn runtime_module(shard: &Shard) -> Result<Vec<u8>> {
    match shard.runtime.runtime_file() {
        None => {
            if !shard.wasm_image.starts_with(b"\0asm") {
                anyhow::bail!("Wasm job payload does not have a valid WebAssembly magic header");
            }
            Ok(shard.wasm_image.clone())
        }
        Some(runtime_file) => {
            let runtime_path = std::path::Path::new("./runtimes").join(runtime_file);
            let runtime = std::fs::read(&runtime_path).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to read declared runtime {}: {}",
                    runtime_path.display(),
                    e
                )
            })?;
            if !runtime.starts_with(b"\0asm") {
                anyhow::bail!(
                    "Declared runtime {} is not valid WebAssembly",
                    runtime_path.display()
                );
            }
            Ok(runtime)
        }
    }
}

fn execution_workspace(shard: &Shard) -> Result<std::path::PathBuf> {
    let base_dir = std::path::Path::new("./swarm-workspaces");
    std::fs::create_dir_all(base_dir)?;
    let canonical_base = base_dir.canonicalize()?;
    let workspace = canonical_base
        .join(shard.parent_task_id.to_string())
        .join(shard.shard_index.to_string())
        .join(uuid::Uuid::new_v4().to_string());
    std::fs::create_dir_all(&workspace)?;

    let canonical_workspace = workspace.canonicalize()?;
    if !canonical_workspace.starts_with(&canonical_base) {
        anyhow::bail!("Execution workspace escaped the configured workspace root");
    }
    Ok(canonical_workspace)
}

pub async fn run_worker(
    shard_id: u64,
    verifying_key: VerifyingKey,
    seed: [u8; 32],
    bootnode: String,
) -> Result<()> {
    let port = 4000 + shard_id as u16;
    let mut p2p_node = SynapseNode::new(port, seed).await?;
    let local_peer_id = *p2p_node.swarm.local_peer_id();

    p2p_node.subscribe("swarm-control-plane")?;

    if let Ok(addr) = bootnode.parse::<libp2p::Multiaddr>() {
        println!("📞 Dialing Orchestration Gateway: {}", addr);
        let _ = p2p_node.swarm.dial(addr);
    } else {
        eprintln!(
            "⚠️ WARNING: Invalid bootnode multiaddress provided. Node will run in isolation."
        );
    }

    let connected_peers = Arc::new(DashSet::new());
    let result_signing_key = SigningKey::from_bytes(&seed);
    let result_public_key = result_signing_key.verifying_key().to_bytes().to_vec();

    let (worker_tx, mut worker_rx) = tokio::sync::mpsc::channel::<NodeCommand>(1000);
    let worker_tx_clone = worker_tx.clone();

    println!(
        "🚀 Swarm Worker Node Active! Shard ID: {} | Peer ID: {}",
        shard_id, local_peer_id
    );

    loop {
        tokio::select! {
            cmd = worker_rx.recv() => {
                if let Some(cmd) = cmd {
                    match cmd {
                        NodeCommand::Unicast(peer, req) => { let _ = p2p_node.send_request(&peer, req); },
                        NodeCommand::Disconnect(peer) => { let _ = p2p_node.swarm.disconnect_peer_id(peer); },
                        _ => {}
                    }
                }
            },
            event = p2p_node.swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(SynapseBehaviorEvent::ReqRes(request_response::Event::Message { peer, message })) => {
                        if let request_response::Message::Request { request: SwarmRequest::DispatchShard(json_payload), channel, .. } = message {
                            let _ = p2p_node.swarm.behaviour_mut().req_res.send_response(channel, SwarmResponse::Ack);

                            let tx_clone = worker_tx_clone.clone();
                            let result_signing_key = result_signing_key.clone();
                            let result_public_key = result_public_key.clone();

                            // PHASE 14: Spawn a dedicated task to handle execution and async file I/O
                            tokio::spawn(async move {
                                if let Ok(signed_payload) = serde_json::from_str::<SignedPayload>(&json_payload) {

                                    // 1. Cryptographic Verification
                                    let message_to_verify = format!("{}:{}", signed_payload.payload_json, signed_payload.expires_at);
                                    let signature_bytes: [u8; 64] = match signed_payload.signature.as_slice().try_into() {
                                        Ok(bytes) => bytes,
                                        Err(_) => {
                                            eprintln!("🚨 SECURITY BREACH: Dispatch payload has an invalid signature length.");
                                            return;
                                        }
                                    };
                                    let signature = Signature::from_bytes(&signature_bytes);
                                    if verifying_key
                                        .verify(message_to_verify.as_bytes(), &signature)
                                        .is_err()
                                    {
                                        eprintln!("🚨 SECURITY BREACH: Invalid dispatch signature from Gateway.");
                                        return;
                                    }

                                    // 2. Expiration Check
                                            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
                                            if current_time > signed_payload.expires_at {
                                                println!("⏳ REJECTED: Dispatch payload expired.");
                                                return;
                                            }

                                            // 3. Deserialize Shard Data
                                            if let Ok(shard_data) = serde_json::from_str::<Shard>(&signed_payload.payload_json) {
                                                println!("⚙️ EXECUTING: Job {} | Shard {}/{}", shard_data.parent_task_id, shard_data.shard_index + 1, shard_data.total_shards);

                                                // 4. Run the sandbox in a unique workspace and extract only its scoped state.
                                                let workspace = match execution_workspace(&shard_data) {
                                                    Ok(workspace) => workspace,
                                                    Err(e) => {
                                                        eprintln!("❌ Failed to create isolated workspace: {e}");
                                                        return;
                                                    }
                                                };
                                                let module_bytes = match runtime_module(&shard_data) {
                                                    Ok(module) => module,
                                                    Err(e) => {
                                                        eprintln!("❌ Runtime resolution failed: {e}");
                                                        let _ = std::fs::remove_dir_all(&workspace);
                                                        return;
                                                    }
                                                };
                                                let mut judge = match Judge::new(None) {
                                                    Ok(judge) => judge,
                                                    Err(e) => {
                                                        eprintln!("❌ Failed to initialize Judge: {e}");
                                                        let _ = std::fs::remove_dir_all(&workspace);
                                                        return;
                                                    }
                                                };
                                                let (execution_result_code, execution_result_hash, new_state) =
                                                    match judge.execute(
                                                        &module_bytes,
                                                        &shard_data.data,
                                                        shard_data.runtime.polyglot_id(),
                                                        &workspace,
                                                    ) {
                                                        Ok(result) => result,
                                                        Err(e) => {
                                                            eprintln!("❌ JUDGE EXECUTION ERROR: {e}");
                                                            (-1, "ERROR".to_string(), None)
                                                        }
                                                    };

                                                // 5. Parse state only from this execution's isolated workspace.
                                                let mut state_delta: BTreeMap<String, String> = BTreeMap::new();
                                                if let Some(state_bytes) = new_state {
                                                    match String::from_utf8(state_bytes)
                                                        .ok()
                                                        .and_then(|contents| serde_json::from_str::<BTreeMap<String, String>>(&contents).ok())
                                                    {
                                                        Some(parsed_delta) => {
                                                            state_delta = parsed_delta;
                                                            println!("📝 CAPTURED: {} state mutations.", state_delta.len());
                                                        }
                                                        None => {
                                                            eprintln!("⚠️ WARNING: Ignoring non-JSON state emitted by isolated workspace.");
                                                        }
                                                    }
                                                }
                                                let _ = std::fs::remove_dir_all(&workspace);

                                                // 6. PHASE 14: Timestamping
                                                let execution_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);

                                                // 7. Construct and Send the new ShardResult
                                                let result_obj = ShardResult {
                                                    job_id: shard_data.parent_task_id,
                                                    shard_index: shard_data.shard_index,
                                                    result: execution_result_code,
                                                    result_hash: execution_result_hash,
                                                    state_delta,
                                                    execution_timestamp
                                                };

                                                let result_message = match shard_result_message(&result_obj) {
                                                    Ok(message) => message,
                                                    Err(e) => {
                                                        eprintln!("⚠️ RESULT SERIALIZATION FAILED: {}", e);
                                                        return;
                                                    }
                                                };
                                                let signed_result = SignedShardResult {
                                                    result: result_obj,
                                                    worker_public_key: result_public_key,
                                                    signature: result_signing_key
                                                        .sign(&result_message)
                                                        .to_bytes()
                                                        .to_vec(),
                                                };
                                                let req = match serde_json::to_string(&signed_result) {
                                                    Ok(s) => SwarmRequest::SubmitResult(s),
                                                    Err(e) => {
                                                        eprintln!("⚠️ SIGNED RESULT SERIALIZATION FAILED: {}", e);
                                                        return;
                                                    }
                                                };

                                                if let Err(e) = tx_clone.try_send(NodeCommand::Unicast(peer, req)) {
                                                    eprintln!("⚠️ BACKPRESSURE ALARM: Failed to send SubmitResult command: {}", e);
                                                } else {
                                                    println!("📤 TRANSMITTED: Result & State Delta sent to Gateway.");
                                                }
                                            }
                                    }
                            });
                        }
                    },
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        println!("📡 CONNECTED: {}", peer_id);
                        connected_peers.insert(peer_id);
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
