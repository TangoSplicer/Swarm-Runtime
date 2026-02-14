import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    # 1. Add the Aggregation Map to AppState
    if "pub stats: Arc<Mutex<SwarmStatus>>," in line:
        new_lines.append(line)
        new_lines.append("    pub aggregations: Arc<DashMap<Uuid, Mutex<Vec<i32>>>>,\n")
    
    # 2. Initialize the Aggregation Map in main()
    elif "let stats = Arc::new(Mutex::new(SwarmStatus {" in line:
        new_lines.append("            let aggregations = Arc::new(DashMap::new());\n")
        new_lines.append(line)
        
    elif "stats: stats.clone()," in line:
        new_lines.append(line)
        new_lines.append("                aggregations: aggregations.clone(),\n")

    # 3. Update the SHARD_RESULT listener to actually sum numbers
    elif 'else if text.starts_with("SHARD_RESULT:") {' in line:
        new_lines.append(line)
        new_lines.append("""                                        let parts: Vec<&str> = text.split(':').collect();
                                        if parts.len() >= 4 {
                                            if let Ok(task_id) = Uuid::parse_str(parts[1]) {
                                                let val: i32 = parts[3].parse().unwrap_or(0);
                                                if let Some(agg_entry) = aggregations.get(&task_id) {
                                                    let mut results = agg_entry.lock().await;
                                                    results.push(val);
                                                    println!("Orchestrator: Aggregating shard result... {} collected", results.len());
                                                }
                                            }
                                        }
""")
        # Skip the old print-only logic we added earlier
        continue
    else:
        new_lines.append(line)

with open(path, "w") as f:
    f.writelines(new_lines)
