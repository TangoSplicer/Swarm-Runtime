import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# Add the logic to handle SHARD_RESULT messages in the P2P loop
new_logic = """
                                    else if text.starts_with("SHARD_RESULT:") {
                                        let parts: Vec<&str> = text.split(':').collect();
                                        if parts.len() >= 4 {
                                            let task_id = parts[1];
                                            let result_val = parts[3];
                                            println!("Orchestrator: Received result from shard: {} = {}", task_id, result_val);
                                            // For now, we print it. In v0.8 we will push to a result map.
                                            if let Some((_, otx)) = pending_clone.remove(task_id) {
                                                let _ = otx.send(result_val.to_string());
                                            }
                                        }
                                    }
"""

# Insert it into the gossipsub event matcher
if "else if text.starts_with(\"TASK_RESULT:\") {" in content:
    updated = content.replace('if text.starts_with("TASK_RESULT:") {', 
                              'if text.starts_with("TASK_RESULT:") {' + new_logic)
    with open(path, "w") as f:
        f.write(updated)
