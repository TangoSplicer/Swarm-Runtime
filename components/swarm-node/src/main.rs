use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::Duration;
use libp2p::swarm::SwarmEvent;
use libp2p::gossipsub;
use futures::StreamExt;
use axum::{extract::{State, Json}, routing::{post, get}, Router, response::Html, http::StatusCode};
use std::sync::Arc;
use std::collections::HashSet;
use dashmap::DashMap;
use uuid::Uuid;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};
use tokio::time::timeout;

use synapse::{SynapseNode, SynapseBehaviorEvent};
use judge::Judge;

mod sharding;
use sharding::{Shard, ShardedDeployRequest};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start { #[arg(long)] shard: Option<u64> },
    Gateway { #[arg(long, default_value = "3000")] port: u16 },
}

#[derive(Serialize, Clone)]
pub struct SwarmStatus { 
    pub version: String, pub role: String, pub peers_count: usize,
    #[serde(skip)] pub peers: HashSet<libp2p::PeerId>,
}

pub struct AppState {
    pub node_tx: tokio::sync::mpsc::UnboundedSender<String>,
    pub aggregations: Arc<DashMap<Uuid, Arc<Mutex<Vec<(u32, i32)>>>>>,
    pub stats: Arc<Mutex<SwarmStatus>>,
    // HEALTH REGISTRY: Map PeerId -> Strike Count
    pub health_registry: Arc<DashMap<libp2p::PeerId, u8>>,
}

#[derive(Serialize)]
struct DeployResponse {
    status: String,
    total_sum: i32,
    breakdown: Vec<(u32, i32)>,
    missing_shards: Vec<u32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Gateway { port } => {
            // --- GATEWAY LOGIC ---
            let mut p2p_node = SynapseNode::new(4000).await?;
            p2p_node.subscribe("swarm-shard-1")?; 
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            
            let aggregations = Arc::new(DashMap::new());
            let stats = Arc::new(Mutex::new(SwarmStatus { 
                version: "0.10.0".to_string(), role: "GATEWAY".to_string(), peers_count: 0, peers: HashSet::new(),
            }));
            let health_registry = Arc::new(DashMap::new());

            let shared_state = Arc::new(AppState { 
                node_tx: tx, 
                aggregations: aggregations.clone(), 
                stats: stats.clone(),
                health_registry: health_registry.clone() 
            });
            
            let (agg_c, stat_c, health_c) = (aggregations.clone(), stats.clone(), health_registry.clone());
            
            // Gateway P2P Loop
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        Some(cmd) = rx.recv() => { let _ = p2p_node.publish_to_topic("swarm-shard-1", cmd); },
                        event = p2p_node.swarm.select_next_some() => {
                            match event {
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                    let text = String::from_utf8_lossy(&message.data);
                                    if text.starts_with("SHARD_RESULT:") {
                                        // Update Health (Strike Removal)
                                        if let Some(source) = message.source {
                                            health_c.remove(&source); 
                                        }

                                        let parts: Vec<&str> = text.split(':').collect();
                                        if parts.len() >= 4 {
                                            if let (Ok(tid), Ok(s_idx), Ok(val)) = (Uuid::parse_str(parts[1]), parts[2].parse::<u32>(), parts[3].parse::<i32>()) {
                                                if let Some(agg_ref) = agg_c.get(&tid) { 
                                                    let mut guard = agg_ref.value().lock().await;
                                                    
                                                    // FIX: IDEMPOTENCY (Prevent "128/164" bug)
                                                    let already_has = guard.iter().any(|(s, _)| *s == s_idx);
                                                    if !already_has {
                                                        guard.push((s_idx, val));
                                                        println!("📝 Accepted Result: Shard {} = {}", s_idx, val);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                    let mut s = stat_c.lock().await;
                                    if s.peers.insert(peer_id) { s.peers_count = s.peers.len(); println!("📡 CONNECTED: {}", peer_id); }
                                    // Reset health on reconnect
                                    health_c.remove(&peer_id);
                                },
                                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                                    let mut s = stat_c.lock().await;
                                    if s.peers.remove(&peer_id) { s.peers_count = s.peers.len(); println!("📉 DISCONNECTED: {}", peer_id); }
                                },
                                _ => {}
                            }
                        }
                    }
                }
            });

            // Gateway HTTP Server
            let app = Router::new().route("/", get(dashboard)).route("/deploy/shard", post(handle_deploy)).with_state(shared_state);
            let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
            println!("Gateway Active: http://localhost:{}", port);
            axum::Server::bind(&addr).serve(app.into_make_service()).await?;
        }
        Commands::Start { shard } => {
            // --- WORKER LOGIC ---
            let shard_id = shard.unwrap_or(1);
            let mut p2p_node = SynapseNode::new(4000 + shard_id as u16).await?;
            p2p_node.subscribe("swarm-shard-1")?;
            println!("=== Worker Live (Shard {}) ===", shard_id);
            
            let _ = p2p_node.swarm.dial("/ip4/127.0.0.1/tcp/4000".parse::<libp2p::Multiaddr>().unwrap());

            loop {
                tokio::select! {
                    // STABILITY FIX: Prevent connection churning
                    _ = tokio::time::sleep(Duration::from_secs(5)) => {
                        if p2p_node.swarm.network_info().num_peers() == 0 {
                             let _ = p2p_node.swarm.dial("/ip4/127.0.0.1/tcp/4000".parse::<libp2p::Multiaddr>().unwrap());
                        }
                    },
                    event = p2p_node.swarm.select_next_some() => {
                        if let SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) = event {
                            let text = String::from_utf8_lossy(&message.data);
                            if text.starts_with("SHARD:") {
                                if let Ok(shard_data) = serde_json::from_str::<Shard>(&text[6..]) {
                                    if let Ok(wasm) = general_purpose::STANDARD.decode(&shard_data.wasm_image) {
                                        println!("⚙️ Executing Shard {}...", shard_data.shard_index);
                                        let mut judge = Judge::new(None).unwrap();
                                        let range = (shard_data.data_range_start, shard_data.data_range_end);
                                        
                                        let res = match judge.execute(&wasm, Some(range)) {
                                            Ok(val) => val,
                                            Err(_) => -1 // Logged by Judge internal
                                        };
                                        
                                        println!("✅ Result: {}", res);
                                        let _ = p2p_node.publish_to_topic("swarm-shard-1", format!("SHARD_RESULT:{}:{}:{}", shard_data.parent_task_id, shard_data.shard_index, res));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

async fn handle_deploy(State(state): State<Arc<AppState>>, Json(payload): Json<ShardedDeployRequest>) -> (StatusCode, Json<DeployResponse>) {
    let task_id = Uuid::new_v4();
    
    // STATE RECOVERY: Filter out Dead Nodes (Strike 2+)
    let all_peers = state.stats.lock().await.peers.clone();
    let active_peers: Vec<_> = all_peers.iter()
        .filter(|p| *state.health_registry.get(p).as_deref().unwrap_or(&0) < 2)
        .collect();
        
    let peer_count = active_peers.len().max(1); // Avoid div by zero
    
    // 1. Initialize result bucket
    let results_store = Arc::new(Mutex::new(Vec::new()));
    state.aggregations.insert(task_id, results_store.clone());
    
    // 2. Dispatch Shards
    let chunk_size = payload.range_size / peer_count as u64;
    for i in 0..peer_count {
        let start = i as u64 * chunk_size;
        let end = if i == (peer_count - 1) { payload.range_size } else { (i as u64 + 1) * chunk_size };
        let shard = Shard { 
            parent_task_id: task_id, shard_index: i as u32, total_shards: peer_count as u32, 
            data_range_start: start, data_range_end: end, wasm_image: payload.wasm_base64.clone() 
        };
        let _ = state.node_tx.send(format!("SHARD:{}", serde_json::to_string(&shard).unwrap()));
    }

    // 3. Robust Aggregation (Wait for 'peer_count', not 'all_peers')
    let timeout_duration = Duration::from_secs(5);
    let aggregation_result = timeout(timeout_duration, async {
        loop {
            let current = results_store.lock().await;
            if current.len() >= peer_count {
                return current.clone();
            }
            drop(current);
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }).await;
    
    // 4. Handle Results
    match aggregation_result {
        Ok(results) => {
            let sum: i32 = results.iter().map(|(_, val)| val).sum();
            (StatusCode::OK, Json(DeployResponse {
                status: "Success".to_string(), total_sum: sum, breakdown: results, missing_shards: vec![],
            }))
        },
        Err(_) => {
            // TIMEOUT CASE
            let received = results_store.lock().await.clone();
            let received_indices: Vec<u32> = received.iter().map(|(idx, _)| *idx).collect();
            let missing_indices: Vec<u32> = (0..peer_count as u32).filter(|i| !received_indices.contains(i)).collect();
            
            // Mark Strikes against missing nodes? 
            // Simplified: We assume missing indices correspond to mapped peers. 
            // In v0.10.0 we just mark ALL currently connected peers as 'Suspicious' if they didn't reply? 
            // No, that's too aggressive. For now, we rely on the next loop to filter.
            
            // Actually, let's mark the missing ones in registry if we could map them. 
            // Since we use gossipsub, we don't know EXACTLY which PeerID maps to ShardID without a table.
            // For Phase 1, we accept the timeout. Phase 2 adds the mapping.

            let sum: i32 = received.iter().map(|(_, val)| val).sum();
            println!("⚠️ Partial Success: Missing {:?}", missing_indices);

            (StatusCode::PARTIAL_CONTENT, Json(DeployResponse { 
                status: "Partial".to_string(), total_sum: sum, breakdown: received, missing_shards: missing_indices 
            }))
        }
    }
}

async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let s = state.stats.lock().await;
    let mut peer_map = String::new();
    for p in s.peers.iter() {
        let health = *state.health_registry.get(p).as_deref().unwrap_or(&0);
        let status_color = if health < 2 { "var(--green)" } else { "#ef4444" };
        
        peer_map.push_str(&format!(
            "<div class='node-card'><div style='display:flex;align-items:center;gap:12px;'><span class='hb-pulse' style='background:{}'></span><span class='node-id'>NODE: {}</span></div></div>", 
            status_color, p
        ));
    }
    let html = format!(
        r##"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1.0">
        <style>
            :root {{ --bg:#0b0f1a; --card:#161b2c; --accent:#3b82f6; --text:#e2e8f0; --green:#10b981; --warn:#f59e0b; }}
            body {{ font-family:sans-serif; background:var(--bg); color:var(--text); padding:15px; display:flex; justify-content:center; }}
            .app {{ width:100%; max-width:450px; }}
            h1 {{ font-size:26px; font-weight:800; color:#38bdf8; margin:0; }}
            .card {{ background:var(--card); padding:20px; border-radius:12px; margin-bottom:15px; border:1px solid #232a3d; position:relative; }}
            .card::before {{ content:''; position:absolute; left:0; top:0; bottom:0; width:4px; }}
            .blue::before {{ background:var(--accent); }}
            .node-card {{ background:#0b0f1a; padding:15px; border-radius:10px; margin-top:10px; border:1px solid #232a3d; }}
            .node-id {{ font-family:monospace; font-size:10px; color:#94a3b8; }}
            .btn {{ background:var(--accent); color:white; border:none; width:100%; padding:16px; border-radius:10px; font-weight:bold; margin-top:15px; cursor:pointer; transition: opacity 0.2s; }}
            .btn:disabled {{ opacity: 0.5; cursor: not-allowed; }}
            .hb-pulse {{ width:10px; height:10px; border-radius:50%; box-shadow:0 0 8px currentColor; }}
            .std-input {{ width: 100%; padding: 10px; background: #0b0f1a; border: 1px solid #334155; color: #fff; border-radius: 6px; margin-bottom: 10px; box-sizing: border-box; }}
            table {{ width: 100%; border-collapse: collapse; margin-top: 10px; }}
            td, th {{ padding: 8px; text-align: left; font-size: 12px; border-bottom: 1px solid #334155; }}
            th {{ color: #94a3b8; }}
        </style></head>
        <body><div class="app">
            <div style="display:flex; justify-content:space-between; align-items:center;"><h1>Swarm Cluster</h1><div style="color:var(--green); font-size:11px; border:1px solid var(--green); padding:2px 8px; border-radius:4px; font-weight:bold;">v0.10.0</div></div>
            <div style="color:#94a3b8; font-size:12px; margin-bottom:25px;">Robustness Phase</div>
            <div class="card blue"><div style="color:#94a3b8; font-size:11px; text-transform:uppercase;">Active Peers</div><div style="font-size:28px; font-weight:700;">{}</div></div>
            
            <div class="card">
                <div style="font-weight:bold; margin-bottom:10px;">🚀 Deploy Wasm</div>
                <input type="file" id="file_in" class="std-input">
                <button id="broadcast-btn" class="btn" onclick="startBroadcast()">BROADCAST TO SWARM</button>
            </div>

            <div class="card" id="res-card" style="display:none;">
                <div id="status-text" style="font-weight:bold; color:var(--green);">✅ Computation Complete</div>
                <div style="font-size:24px; font-weight:bold; margin:10px 0;" id="total-sum"></div>
                <table id="res-table">
                    <thead><tr><th>Shard ID</th><th>Result</th></tr></thead>
                    <tbody id="res-body"></tbody>
                </table>
            </div>
            
            <div class="card" style="padding:15px;"><div style="color:#94a3b8; font-size:11px; text-transform:uppercase;">Live Peer Map</div>{}</div>
        </div>
        <script>
            let b64 = "";

            document.getElementById('file_in').addEventListener('change', function(e) {{
                const file = e.target.files[0];
                if (!file) return;
                const reader = new FileReader();
                reader.onload = function(ev) {{ b64 = ev.target.result.split(',')[1]; }};
                reader.readAsDataURL(file);
            }});

            async function startBroadcast() {{
                if(!b64) return alert("Select a file first!");
                
                const btn = document.getElementById('broadcast-btn');
                const resCard = document.getElementById('res-card');
                const statusText = document.getElementById('status-text');
                
                // UI STATE: BUSY
                btn.disabled = true;
                btn.innerText = "Broadcasting... (Max 5s)";
                // Don't hide the card immediately if it has previous results, just update status
                // resCard.style.display = 'none'; 
                
                try {{
                    const res = await fetch('/deploy/shard', {{
                        method: 'POST',
                        headers: {{ 'Content-Type': 'application/json' }},
                        body: JSON.stringify({{ wasm_base64: b64, range_size: 100 }})
                    }});
                    
                    if (!res.ok) throw new Error("Server Error: " + res.status);

                    const data = await res.json();
                    resCard.style.display = 'block';
                    document.getElementById('total-sum').innerText = "Total: " + data.total_sum;
                    
                    if (data.status === "Success") {{
                        statusText.innerText = "✅ Computation Complete";
                        statusText.style.color = "var(--green)";
                    }} else {{
                        statusText.innerText = "⚠️ Partial Result (" + data.status + ")";
                        statusText.style.color = "var(--warn)";
                    }}
                    
                    const tbody = document.getElementById('res-body');
                    tbody.innerHTML = "";
                    data.breakdown.forEach(row => {{
                        const tr = document.createElement('tr');
                        tr.innerHTML = `<td>Shard ${{row[0]}}</td><td style="font-family:monospace; color:#38bdf8;">${{row[1]}}</td>`;
                        tbody.appendChild(tr);
                    }});
                    
                }} catch (e) {{
                    alert("Broadcast Failed: " + e);
                    statusText.innerText = "❌ Error: " + e;
                    statusText.style.color = "#ef4444";
                    resCard.style.display = 'block'; // Show error in card
                }} finally {{
                    btn.disabled = false;
                    btn.innerText = "BROADCAST TO SWARM";
                }}
            }}
            
            // Reload logic: Only reload if NO file is selected to prevent clearing input
            setInterval(() => {{ if(!b64 && document.getElementById('file_in').value === "") location.reload(); }}, 5000);
        </script></body></html>"##, s.peers_count, peer_map
    );
    Html(html)
}
