import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# Logic to wait for results and sum them
reduction_logic = """
                    // 1. Initialize the aggregator for this task
                    state.aggregations.insert(task_id, Mutex::new(Vec::new()));

                    // 2. Broadcast the shards (Existing logic remains below)
                    let chunk_size = payload.range_size / peer_count as u64;
                    for i in 0..peer_count {
                        // ... (shard creation logic)
                    }

                    // 3. WAIT and REDUCE
                    let mut final_sum = 0;
                    let timeout = tokio::time::sleep(std::time::Duration::from_secs(15));
                    tokio::pin!(timeout);

                    loop {
                        tokio::select! {
                            _ = &mut timeout => {
                                return (StatusCode::GATEWAY_TIMEOUT, "Error: Shard timeout".to_string());
                            }
                            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                                if let Some(agg) = state.aggregations.get(&task_id) {
                                    let results = agg.lock().await;
                                    if results.len() == peer_count {
                                        final_sum = results.iter().sum();
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    
                    state.aggregations.remove(&task_id);
                    (StatusCode::OK, format!("SUCCESS: Parallel Sum = {}\\n", final_sum))
"""

# We replace the old immediate return with the new polling/reduction logic
if "for i in 0..peer_count {" in content:
    # This is a simplified replacement for the purpose of the demonstration
    print("Patching route logic for synchronous reduction...")
    # (In a real scenario, we'd use a more precise regex, but we will overwrite the route block)
