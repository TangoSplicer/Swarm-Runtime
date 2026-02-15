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
    pub version: String, 
    pub role: String, 
    pub peers_count: usize,
    #[serde(skip)]
    pub peers: HashSet<libp2p::PeerId>,
}

pub struct AppState {
    pub node_tx: tokio::sync::mpsc::UnboundedSender<String>,
    pub aggregations: Arc<DashMap<Uuid, Arc<Mutex<Vec<i32>>>>>,
    pub stats: Arc<Mutex<SwarmStatus>>,
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
                version: "0.8.9".to_string(), role: "GATEWAY".to_string(), peers_count: 0, peers: HashSet::new(),
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
                                        if let (Ok(tid), Ok(val)) = (Uuid::parse_str(parts[1]), parts[3].parse::<i32>()) {
                                            if let Some(agg_ref) = agg_c.get(&tid) { agg_ref.value().lock().await.push(val); }
                                        }
                                    }
                                },
                                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                    let mut s = stat_c.lock().await;
                                    if s.peers.insert(peer_id) { 
                                        s.peers_count = s.peers.len(); 
                                        println!("📡 NEW CONNECTION: {}", peer_id); 
                                    }
                                },
                                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                                    let mut s = stat_c.lock().await;
                                    if s.peers.remove(&peer_id) { 
                                        s.peers_count = s.peers.len(); 
                                        println!("📉 PEER DROPPED: {}", peer_id); 
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                }
            });
            let app = Router::new()
                .route("/", get(dashboard))
                .route("/deploy/shard", post(handle_deploy))
                .with_state(shared_state);
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
                                        let mut judge = Judge::new(None).unwrap();
                                        let res = judge.execute(&wasm, Some((shard_data.data_range_start, shard_data.data_range_end))).unwrap_or(-1);
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

async fn handle_deploy(State(state): State<Arc<AppState>>, Json(payload): Json<ShardedDeployRequest>) -> (StatusCode, String) {
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
    (StatusCode::OK, "Broadcast Initiated".to_string())
}

async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let s = state.stats.lock().await;
    let mut peer_map = String::new();
    for (i, p) in s.peers.iter().enumerate() {
        peer_map.push_str(&format!(
            "<div style='background:#0b0f1a;padding:12px;border-radius:8px;margin-top:10px;border:1px solid #232a3d;'>
                <div style='display:flex;align-items:center;gap:10px;'><span class='pulse'></span><span style='font-size:10px;color:#94a3b8;'>NODE ID: {}</span></div>
                <div style='width:100%;height:4px;background:#232a3d;border-radius:2px;margin-top:8px;'><div id='b-{}' style='width:0%;height:100%;background:#10b981;transition:width 0.5s;'></div></div>
            </div>", p, i
        ));
    }
    let html = format!(
        r##"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1.0">
        <style>
            :root {{ --bg:#0b0f1a; --card:#161b2c; --accent:#3b82f6; --text:#e2e8f0; }}
            body {{ font-family:sans-serif; background:var(--bg); color:var(--text); padding:15px; display:flex; justify-content:center; }}
            .app {{ width:100%; max-width:450px; }}
            h1 {{ font-size:26px; font-weight:800; color:#38bdf8; margin:0; }}
            .card {{ background:var(--card); padding:20px; border-radius:12px; margin-bottom:15px; border:1px solid #232a3d; position:relative; }}
            .card::before {{ content:''; position:absolute; left:0; top:0; bottom:0; width:4px; }}
            .blue::before {{ background:var(--accent); }}
            .purple::before {{ background:#a855f7; }}
            .pulse {{ width:8px; height:8px; background:#10b981; border-radius:50%; animation: p 1.5s infinite; }}
            @keyframes p {{ 0% {{ opacity:1; }} 50% {{ opacity:0.3; }} 100% {{ opacity:1; }} }}
            .upload {{ border:2px dashed #334155; padding:25px; border-radius:10px; text-align:center; cursor:pointer; color:#94a3b8; margin-top:10px; }}
            .btn {{ background:var(--accent); color:white; border:none; width:100%; padding:14px; border-radius:8px; font-weight:bold; margin-top:15px; cursor:pointer; }}
            #log {{ background:black; color:#10b981; padding:10px; font-family:monospace; height:80px; overflow-y:auto; border-radius:8px; border:1px solid #232a3d; margin-top:15px; }}
        </style></head>
        <body><div class="app">
            <div style="display:flex; justify-content:space-between; align-items:center;"><h1>Swarm Cluster</h1><div style="color:#10b981; font-size:11px; border:1px solid #10b981; padding:2px 8px; border-radius:4px; font-weight:bold;">v0.8.9</div></div>
            <div style="color:#94a3b8; font-size:12px; margin-bottom:25px;">Distributed Compute Engine</div>
            <div class="card blue"><div style="color:#94a3b8; font-size:11px; text-transform:uppercase;">Connected Peers</div><div style="font-size:28px; font-weight:700;">{}</div></div>
            <div class="card purple"><div style="color:#94a3b8; font-size:11px; text-transform:uppercase;">Node Role</div><div style="font-size:28px; font-weight:700; color:#a855f7;">GATEWAY</div></div>
            <div class="card">
                <div style="font-weight:bold; margin-bottom:10px;">🚀 Deploy Wasm</div>
                <input type="file" id="wi" style="display:none">
                <div class="upload" id="uz" onclick="document.getElementById('wi').click()">Select .wasm module</div>
                <button class="btn" id="bb" onclick="deploy()">BROADCAST TO SWARM</button>
            </div>
            <div class="card" style="padding:15px;"><div style="color:#94a3b8; font-size:11px; text-transform:uppercase;">Live Peer Map</div>{}</div>
            <div id="log">> Cluster ready.</div>
        </div>
        <script>
            let b64 = "";
            const wi = document.getElementById('wi');
            const uz = document.getElementById('uz');

            wi.addEventListener('change', async function() {{
                const file = this.files[0];
                if (!file) return;
                
                uz.innerText = "⏳ Reading " + file.name + "...";
                
                try {{
                    b64 = await new Promise((resolve, reject) => {{
                        const reader = new FileReader();
                        reader.onload = () => resolve(reader.result.split(',')[1]);
                        reader.onerror = reject;
                        reader.readAsDataURL(file);
                    }});
                    uz.innerText = "✅ READY: " + file.name;
                    uz.style.color = "#10b981";
                    uz.style.borderColor = "#10b981";
                }} catch (e) {{
                    uz.innerText = "❌ Load Failed";
                    console.error(e);
                }}
            }});

            async function deploy() {{
                if(!b64) return alert("Select .wasm file first!");
                document.querySelectorAll('[id^="b-"]').forEach(b => b.style.width = "100%");
                const res = await fetch('/deploy/shard', {{
                    method:'POST',
                    headers:{{'Content-Type':'application/json'}},
                    body: JSON.stringify({{wasm_base64:b64, range_size: 100}})
                }});
                const text = await res.text();
                document.getElementById('log').innerHTML += "<br>> " + text;
            }}
            setInterval(() => {{ if(!b64 && !wi.files.length) location.reload(); }}, 5000);
        </script></body></html>"##, s.peers_count, peer_map
    );
    Html(html)
}
