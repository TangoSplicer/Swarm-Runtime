import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    lines = f.readlines()

new_lines = []
skip = False

for line in lines:
    # Remove any existing dashboard function to prevent duplication
    if "async fn dashboard" in line:
        skip = True
    if skip:
        if line.strip() == "}":
            skip = False
        continue
    
    # Locate the Router definition and replace it entirely
    if "let app = Router::new()" in line:
        new_lines.append("""            let app = Router::new()
                .route("/", get(dashboard))
                .route("/deploy/shard", post(|State(state): State<Arc<AppState>>, Json(payload): Json<ShardedDeployRequest>| async move {
                    let task_id = Uuid::new_v4();
                    let peer_count = state.stats.lock().await.peers.len().max(1);
                    state.aggregations.insert(task_id, Arc::new(Mutex::new(Vec::new())));

                    println!("Orchestrator: Splitting task across {} peers", peer_count);
                    let chunk_size = payload.range_size / peer_count as u64;
                    for i in 0..peer_count {
                        let start = i as u64 * chunk_size;
                        let end = if i == (peer_count - 1) as usize { payload.range_size } else { (i as u64 + 1) * chunk_size };
                        let shard = Shard { parent_task_id: task_id, shard_index: i as u32, total_shards: peer_count as u32, data_range_start: start, data_range_end: end, wasm_image: payload.wasm_base64.clone() };
                        let _ = state.node_tx.send(format!("SHARD:{}", serde_json::to_string(&shard).unwrap()));
                    }

                    for _ in 0..100 {
                        if let Some(agg_ref) = state.aggregations.get(&task_id) {
                            let results = agg_ref.value().lock().await;
                            if results.len() >= peer_count {
                                let sum: i32 = results.iter().sum();
                                return (StatusCode::OK, format!("SUCCESS: Parallel Sum = {}\\n", sum));
                            }
                        }
                        tokio::time::sleep(Duration::from_millis(200)).await;
                    }
                    (StatusCode::GATEWAY_TIMEOUT, "Timeout waiting for shards\\n".to_string())
                }))
                .with_state(shared_state);\n""")
        continue
        
    # Skip lines that were part of the old broken router block
    if ".route(" in line and (".route(\"/\", get(dashboard))" in line or "/deploy/shard" in line):
        continue
    if ".with_state(shared_state);" in line:
        continue

    new_lines.append(line)

# Append the full, styled Dashboard function with Execution Log
new_lines.append("""
async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let s = state.stats.lock().await;
    let peer_list: String = s.peers.iter().map(|p| format!("<li>{}</li>", p)).collect();
    
    let html = format!(
        "<!DOCTYPE html>
        <html>
        <head>
            <title>Swarm Dashboard v0.8.0</title>
            <style>
                body {{ font-family: sans-serif; background: #0f172a; color: #f8fafc; padding: 20px; line-height: 1.6; }}
                .container {{ max-width: 900px; margin: auto; }}
                .header {{ display: flex; justify-content: space-between; border-bottom: 2px solid #334155; padding-bottom: 10px; }}
                .badge {{ background: #22c55e; color: white; padding: 5px 15px; border-radius: 20px; font-size: 12px; height: fit-content; }}
                .grid {{ display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px; margin-top: 20px; }}
                .card {{ background: #1e293b; padding: 15px; border-radius: 10px; border: 1px solid #334155; text-align: center; }}
                .card h4 {{ margin: 0; color: #94a3b8; font-size: 12px; }}
                .card div {{ font-size: 24px; font-weight: bold; color: #38bdf8; margin-top: 5px; }}
                .logs {{ background: #000; color: #00ff00; padding: 15px; border-radius: 8px; font-family: monospace; height: 150px; overflow-y: auto; margin-top: 20px; border: 1px solid #334155; }}
                .peer-list {{ margin-top: 20px; font-size: 14px; color: #cbd5e1; }}
            </style>
        </head>
        <body>
            <div class='container'>
                <div class='header'>
                    <h1>🐝 Swarm Cluster</h1>
                    <div class='badge'>v0.8.0 - ONLINE</div>
                </div>
                <div class='grid'>
                    <div class='card'><h4>ROLE</h4><div>ORCHESTRATOR</div></div>
                    <div class='card'><h4>PEERS</h4><div>{}</div></div>
                    <div class='card'><h4>TASKS</h4><div>ACTIVE</div></div>
                </div>
                <h3>Live Node Registry</h3>
                <div class='peer-list'><ul>{}</ul></div>
                <h3>Execution Logs</h3>
                <div class='logs'>
                    > [SYSTEM] Swarm v0.8.0 initialized.<br>
                    > [NETWORK] Auto-discovery prober active (Ports 4000-4005).<br>
                    > [READY] Waiting for sharded Wasm deployments...
                </div>
            </div>
        </body>
        </html>",
        s.peers.len(), peer_list
    );
    Html(html)
}
""")

with open(path, "w") as f:
    f.writelines(new_lines)
