#!/bin/bash

cd components/swarm-node/src || exit

# ==========================================
# 1. ADD SYNC COMMAND TO TYPES
# ==========================================
sed -i 's/Broadcast(String),/Broadcast(String),\n    GatewaySync(String),/g' types.rs

# ==========================================
# 2. SUBSCRIBE GATEWAYS TO SYNC TOPIC
# ==========================================
sed -i 's/p2p_node.subscribe("swarm-control-plane")?;/p2p_node.subscribe("swarm-control-plane")?;\n    p2p_node.subscribe("swarm-gateway-sync")?;/g' gateway.rs

# ==========================================
# 3. ROUTE THE SYNC COMMAND TO LIBP2P
# ==========================================
sed -i 's/NodeCommand::Broadcast(msg) => { let _ = p2p_node.publish_to_topic("swarm-control-plane", msg); },/NodeCommand::Broadcast(msg) => { let _ = p2p_node.publish_to_topic("swarm-control-plane", msg); },\n                        NodeCommand::GatewaySync(msg) => { let _ = p2p_node.publish_to_topic("swarm-gateway-sync", msg); },/g' gateway.rs

# ==========================================
# 4. BROADCAST STATE HASHES UPON BFT CONSENSUS
# ==========================================
sed -i 's/contract_states_c.insert(contract_key, final_hash.clone());/contract_states_c.insert(contract_key.clone(), final_hash.clone());\n                                                        let sync_msg = format!("SYNC_STATE:{\\"k\\":\\"{}\\",\\"v\\":\\"{}\\"}", contract_key, final_hash);\n                                                        let _ = tx.try_send(NodeCommand::GatewaySync(sync_msg));/g' gateway.rs

# ==========================================
# 5. ASYNC NON-BLOCKING RECEIVER (CLONE & RELEASE)
# ==========================================
# We insert this right above the Telemetry (TEL:) receiver in the Gossipsub event match
sed -i '/if text.starts_with("TEL:") {/i \
                            if text.starts_with("SYNC_STATE:") {\n                                let payload = text[11..].to_string();\n                                let cs_clone = contract_states_c.clone();\n                                tokio::spawn(async move {\n                                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&payload) {\n                                        if let (Some(k), Some(v)) = (parsed.get("k").and_then(|v| v.as_str()), parsed.get("v").and_then(|v| v.as_str())) {\n                                            cs_clone.insert(k.to_string(), v.to_string());\n                                            println!("🔄 FEDERATION: Synced State Hash [{}] from Peer Gateway", \&v[..8]);\n                                        }\n                                    }\n                                });\n                            }' gateway.rs

echo "✅ Active-Active Gateway Federation Sync Plane injected."
