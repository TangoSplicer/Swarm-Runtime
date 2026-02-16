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
}

#[derive(Serialize)]
struct DeployResponse {
    status: String,
    total_sum: i32,
    breakdown: Vec<(u32, i32)>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Gateway { port } => {
            let mut p2p_node = SynapseNode::new(4000).await?;
            p2p_node.subscribe("swarm-shard-1")?; 
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            let aggregations = Arc::new(DashMap::new());
            let stats = Arc::new(Mutex::new(SwarmStatus { 
                version: "0.9.3".to_string(), role: "GATEWAY".to_string(), peers_count: 0, peers: HashSet::new(),
            }));
            let shared_state = Arc::new(AppState { node_tx: tx, aggregations: aggregations.clone(), stats: stats.clone() });
            
            let (agg_c, stat_c) = (aggregations.clone(), stats.clone());
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        Some(cmd) = rx.recv() => { let _ = p2p_node.publish_to_topic("swarm-shard-1", cmd); },
                        event = p2p_node.swarm.select_next_some() => {
                            match event {
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                    let text = String::from_utf8_lossy(&message.data);
                                    if text.starts_with("SHARD_RESULT:") {
                                        let parts: Vec<&str> = text.split(':').collect();
                                        if parts.len() >= 4 {
                                            if let (Ok(tid), Ok(s_idx), Ok(val)) = (Uuid::parse_str(parts[1]), parts[2].parse::<u32>(), parts[3].parse::<i32>()) {
                                                if let Some(agg_ref) = agg_c.get(&tid) { 
                                                    agg_ref.value().lock().await.push((s_idx, val)); 
                                                }
                                            }
                                        }
                                    }
                                },
                                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                    let mut s = stat_c.lock().await;
                                    if s.peers.insert(peer_id) { s.peers_count = s.peers.len(); println!("📡 NEW CONNECTION: {}", peer_id); }
                                },
                                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                                    let mut s = stat_c.lock().await;
                                    if s.peers.remove(&peer_id) { s.peers_count = s.peers.len(); println!("📉 PEER DROPPED: {}", peer_id); }
                                },
                                _ => {}
                            }
                        }
                    }
                }
            });
            let app = Router::new().route("/", get(dashboard)).route("/deploy/shard", post(handle_deploy)).with_state(shared_state);
            let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
            println!("Gateway Active: http://localhost:{}", port);
            axum::Server::bind(&addr).serve(app.into_make_service()).await?;
        }
        Commands::Start { shard } => {
            let shard_id = shard.unwrap_or(1);
            let mut p2p_node = SynapseNode::new(4000 + shard_id as u16).await?;
            p2p_node.subscribe("swarm-shard-1")?;
            println!("=== Worker Live (Shard {}) ===", shard_id);
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(5)) => {
                        let _ = p2p_node.swarm.dial("/ip4/127.0.0.1/tcp/4000".parse::<libp2p::Multiaddr>().unwrap());
                    },
                    event = p2p_node.swarm.select_next_some() => {
                        if let SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) = event {
                            let text = String::from_utf8_lossy(&message.data);
                            if text.starts_with("SHARD:") {
                                if let Ok(shard_data) = serde_json::from_str::<Shard>(&text[6..]) {
                                    if let Ok(wasm) = general_purpose::STANDARD.decode(&shard_data.wasm_image) {
                                        // EXECUTION LOGIC
                                        let mut judge = Judge::new(None).unwrap();
                                        // Explicitly cast ranges to what our Judge expects
                                        let range = (shard_data.data_range_start, shard_data.data_range_end);
                                        
                                        let res = match judge.execute(&wasm, Some(range)) {
                                            Ok(val) => val,
                                            Err(e) => {
                                                println!("❌ EXECUTION ERROR (Shard {}): {:?}", shard_data.shard_index, e);
                                                -1
                                            }
                                        };
                                        
                                        println!("✅ Computed Shard {}: Result = {}", shard_data.shard_index, res);
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
    let peer_count = state.stats.lock().await.peers_count.max(1);
    
    state.aggregations.insert(task_id, Arc::new(Mutex::new(Vec::new())));
    
    let chunk_size = payload.range_size / peer_count as u64;
    for i in 0..peer_count {
        let start = i as u64 * chunk_size;
        let end = if i == (peer_count - 1) as usize { payload.range_size } else { (i as u64 + 1) * chunk_size };
        let shard = Shard { parent_task_id: task_id, shard_index: i as u32, total_shards: peer_count as u32, data_range_start: start, data_range_end: end, wasm_image: payload.wasm_base64.clone() };
        let _ = state.node_tx.send(format!("SHARD:{}", serde_json::to_string(&shard).unwrap()));
    }

    // Wait for results
    for _ in 0..100 { 
        if let Some(agg_ref) = state.aggregations.get(&task_id) {
            let results = agg_ref.value().lock().await;
            if results.len() >= peer_count {
                let sum: i32 = results.iter().map(|(_, val)| val).sum();
                return (StatusCode::OK, Json(DeployResponse {
                    status: "Success".to_string(),
                    total_sum: sum,
                    breakdown: results.clone()
                }));
            }
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    
    (StatusCode::GATEWAY_TIMEOUT, Json(DeployResponse { status: "Timeout".to_string(), total_sum: 0, breakdown: vec![] }))
}

async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let s = state.stats.lock().await;
    let mut peer_map = String::new();
    for p in s.peers.iter() {
        peer_map.push_str(&format!(
            "<div class='node-card'><div style='display:flex;align-items:center;gap:12px;'><span class='hb-pulse'></span><span class='node-id'>NODE: {}</span></div></div>", p
        ));
    }
    let html = format!(
        r##"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1.0">
        <style>
            :root {{ --bg:#0b0f1a; --card:#161b2c; --accent:#3b82f6; --text:#e2e8f0; --green:#10b981; }}
            body {{ font-family:sans-serif; background:var(--bg); color:var(--text); padding:15px; display:flex; justify-content:center; }}
            .app {{ width:100%; max-width:450px; }}
            h1 {{ font-size:26px; font-weight:800; color:#38bdf8; margin:0; }}
            .card {{ background:var(--card); padding:20px; border-radius:12px; margin-bottom:15px; border:1px solid #232a3d; position:relative; }}
            .card::before {{ content:''; position:absolute; left:0; top:0; bottom:0; width:4px; }}
            .blue::before {{ background:var(--accent); }}
            .purple::before {{ background:#a855f7; }}
            .hb-pulse {{ width:10px; height:10px; background:var(--green); border-radius:50%; animation: p 2s infinite; box-shadow:0 0 8px var(--green); }}
            @keyframes p {{ 0%, 100% {{ opacity:1; scale:1; }} 50% {{ opacity:0.4; scale:0.9; }} }}
            .node-card {{ background:#0b0f1a; padding:15px; border-radius:10px; margin-top:10px; border:1px solid #232a3d; }}
            .node-id {{ font-family:monospace; font-size:10px; color:#94a3b8; overflow:hidden; text-overflow:ellipsis; }}
            .btn {{ background:var(--accent); color:white; border:none; width:100%; padding:16px; border-radius:10px; font-weight:bold; margin-top:15px; cursor:pointer; }}
            
            .std-input {{ width: 100%; padding: 10px; background: #0b0f1a; border: 1px solid #334155; color: #fff; border-radius: 6px; margin-bottom: 10px; box-sizing: border-box; }}
            
            table {{ width: 100%; border-collapse: collapse; margin-top: 10px; }}
            td, th {{ padding: 8px; text-align: left; font-size: 12px; border-bottom: 1px solid #334155; }}
            th {{ color: #94a3b8; }}
        </style></head>
        <body><div class="app">
            <div style="display:flex; justify-content:space-between; align-items:center;"><h1>Swarm Cluster</h1><div style="color:var(--green); font-size:11px; border:1px solid var(--green); padding:2px 8px; border-radius:4px; font-weight:bold;">v0.9.3</div></div>
            <div style="color:#94a3b8; font-size:12px; margin-bottom:25px;">Distributed Compute Engine</div>
            <div class="card blue"><div style="color:#94a3b8; font-size:11px; text-transform:uppercase;">Connected Peers</div><div style="font-size:28px; font-weight:700;">{}</div></div>
            
            <div class="card">
                <div style="font-weight:bold; margin-bottom:10px;">🚀 Deploy Wasm</div>
                <input type="file" id="file_in" class="std-input">
                <button class="btn" onclick="startBroadcast()">BROADCAST TO SWARM</button>
            </div>

            <div class="card" id="res-card" style="display:none;">
                <div style="font-weight:bold; color:var(--green);">✅ Computation Complete</div>
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
                document.getElementById('res-card').style.display = 'none';
                
                try {{
                    const res = await fetch('/deploy/shard', {{
                        method: 'POST',
                        headers: {{ 'Content-Type': 'application/json' }},
                        body: JSON.stringify({{ wasm_base64: b64, range_size: 100 }})
                    }});
                    
                    const data = await res.json();
                    document.getElementById('res-card').style.display = 'block';
                    document.getElementById('total-sum').innerText = "Total: " + data.total_sum;
                    
                    const tbody = document.getElementById('res-body');
                    tbody.innerHTML = "";
                    data.breakdown.forEach(row => {{
                        const tr = document.createElement('tr');
                        tr.innerHTML = `<td>Shard ${{row[0]}}</td><td style="font-family:monospace; color:#38bdf8;">${{row[1]}}</td>`;
                        tbody.appendChild(tr);
                    }});
                    
                }} catch (e) {{
                    alert("Error: " + e);
                }}
            }}
            
            setInterval(() => {{ if(!b64) location.reload(); }}, 5000);
        </script></body></html>"##, s.peers_count, peer_map
    );
    Html(html)
}
