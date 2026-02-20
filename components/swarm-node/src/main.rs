use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::{Duration, Instant};
use libp2p::{swarm::SwarmEvent, gossipsub, mdns, identify};
use futures::StreamExt;
use axum::{
    extract::{State, Json, Path}, 
    routing::{post, get}, 
    Router, 
    response::Html, 
    http::StatusCode
};
use std::sync::Arc;
use std::collections::HashSet;
use dashmap::DashMap;
use uuid::Uuid;
use tokio::sync::Mutex;
use serde::Serialize;
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

// NEW: Job tracking state
pub struct JobState {
    pub expected_shards: usize,
    pub results: Vec<(u32, i32)>,
    pub created_at: Instant,
}

pub struct AppState {
    pub node_tx: tokio::sync::mpsc::UnboundedSender<String>,
    pub jobs: Arc<DashMap<Uuid, Arc<Mutex<JobState>>>>, // Replaced 'aggregations'
    pub stats: Arc<Mutex<SwarmStatus>>,
    pub health_registry: Arc<DashMap<libp2p::PeerId, u8>>,
    pub pending_dials: Arc<DashMap<libp2p::PeerId, Instant>>,
}

#[derive(Serialize)]
struct JobSubmitResponse {
    job_id: String,
    status: String,
}

#[derive(Serialize)]
struct JobStatusResponse {
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
            let mut p2p_node = SynapseNode::new(4000).await?;
            let local_peer_id = *p2p_node.swarm.local_peer_id();
            p2p_node.subscribe("swarm-shard-1")?; 
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            
            let jobs = Arc::new(DashMap::new());
            let stats = Arc::new(Mutex::new(SwarmStatus { 
                version: "0.12.0".to_string(), role: "GATEWAY".to_string(), peers_count: 0, peers: HashSet::new(),
            }));
            let health_registry = Arc::new(DashMap::new());
            let pending_dials = Arc::new(DashMap::<libp2p::PeerId, Instant>::new());
            
            let shared_state = Arc::new(AppState { 
                node_tx: tx, jobs: jobs.clone(), stats: stats.clone(), 
                health_registry: health_registry.clone(), pending_dials: pending_dials.clone()
            });
            let (jobs_c, stat_c, health_c, pending_c) = (jobs.clone(), stats.clone(), health_registry.clone(), pending_dials.clone());
            
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        Some(cmd) = rx.recv() => { let _ = p2p_node.publish_to_topic("swarm-shard-1", cmd); },
                        event = p2p_node.swarm.select_next_some() => {
                            match event {
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::Mdns(mdns::Event::Discovered(list))) => {
                                    for (peer_id, multiaddr) in list {
                                        if !p2p_node.swarm.is_connected(&peer_id) {
                                            if local_peer_id >= peer_id { continue; }
                                            if let Some(last) = pending_c.get(&peer_id) {
                                                if last.elapsed() < Duration::from_secs(5) { continue; }
                                            }
                                            pending_c.insert(peer_id, Instant::now());
                                            println!("🔍 Gateway (Leader): Dialing {}...", peer_id);
                                            p2p_node.swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr.clone());
                                            let _ = p2p_node.swarm.dial(multiaddr);
                                        }
                                    }
                                },
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::Identify(identify::Event::Received { peer_id, info })) => {
                                    println!("🆔 IDENTIFY: Connected to {} ({})", peer_id, info.agent_version);
                                },
                                SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                    let text = String::from_utf8_lossy(&message.data);
                                    if text.starts_with("SHARD_RESULT:") {
                                        if let Some(source) = message.source { health_c.remove(&source); }
                                        let parts: Vec<&str> = text.split(':').collect();
                                        if parts.len() >= 4 {
                                            if let (Ok(tid), Ok(s_idx), Ok(val)) = (Uuid::parse_str(parts[1]), parts[2].parse::<u32>(), parts[3].parse::<i32>()) {
                                                if let Some(job_ref) = jobs_c.get(&tid) { 
                                                    let mut guard = job_ref.value().lock().await;
                                                    if !guard.results.iter().any(|(s, _)| *s == s_idx) {
                                                        guard.results.push((s_idx, val));
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
                                    health_c.remove(&peer_id);
                                    pending_c.remove(&peer_id);
                                },
                                SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                                    let mut s = stat_c.lock().await;
                                    if s.peers.remove(&peer_id) { 
                                        s.peers_count = s.peers.len(); 
                                        println!("📉 DISCONNECTED: {} (Cause: {:?})", peer_id, cause); 
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                }
            });

            // V0.12.0 HEADLESS API ROUTER
            let app = Router::new()
                .route("/", get(dashboard))
                .route("/api/v1/jobs", post(submit_job))
                .route("/api/v1/jobs/:id", get(get_job_status))
                .with_state(shared_state);
                
            let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
            println!("Gateway Active: http://localhost:{}", port);
            println!("API Ready: POST /api/v1/jobs");
            axum::Server::bind(&addr).serve(app.into_make_service()).await?;
        }
        Commands::Start { shard } => {
            let shard_id = shard.unwrap_or(1);
            let port = 4000 + shard_id as u16;
            let mut p2p_node = SynapseNode::new(port).await?;
            let local_peer_id = *p2p_node.swarm.local_peer_id();
            p2p_node.subscribe("swarm-shard-1")?;
            println!("=== Worker Live (Shard {}) on Port {} ===", shard_id, port);
            
            let pending_dials = Arc::new(DashMap::<libp2p::PeerId, Instant>::new());
            let pending_c = pending_dials.clone();

            loop {
                tokio::select! {
                    event = p2p_node.swarm.select_next_some() => {
                        match event {
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Mdns(mdns::Event::Discovered(list))) => {
                                for (peer_id, multiaddr) in list {
                                    if !p2p_node.swarm.is_connected(&peer_id) {
                                        if local_peer_id >= peer_id { continue; }
                                        if let Some(last) = pending_c.get(&peer_id) {
                                            if last.elapsed() < Duration::from_secs(5) { continue; }
                                        }
                                        pending_c.insert(peer_id, Instant::now());
                                        println!("🔍 Worker (Leader): Dialing {}...", peer_id);
                                        p2p_node.swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr.clone());
                                        let _ = p2p_node.swarm.dial(multiaddr);
                                    }
                                }
                            },
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Identify(identify::Event::Received { peer_id, info })) => {
                                println!("🆔 IDENTIFY: Connected to {} ({})", peer_id, info.agent_version);
                            },
                            SwarmEvent::Behaviour(SynapseBehaviorEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                let text = String::from_utf8_lossy(&message.data);
                                if text.starts_with("SHARD:") {
                                    if let Ok(shard_data) = serde_json::from_str::<Shard>(&text[6..]) {
                                        if let Ok(wasm) = general_purpose::STANDARD.decode(&shard_data.wasm_image) {
                                            println!("⚙️ Executing Shard {}...", shard_data.shard_index);
                                            let mut judge = Judge::new(None).unwrap();
                                            let range = (shard_data.data_range_start, shard_data.data_range_end);
                                            let res = match judge.execute(&wasm, Some(range)) {
                                                Ok(val) => val,
                                                Err(e) => { println!("❌ Wasm Error: {}", e); -1 }
                                            };
                                            println!("✅ Result: {}", res);
                                            let _ = p2p_node.publish_to_topic("swarm-shard-1", format!("SHARD_RESULT:{}:{}:{}", shard_data.parent_task_id, shard_data.shard_index, res));
                                        }
                                    }
                                }
                            },
                            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                println!("📡 CONNECTED: {}", peer_id);
                                pending_c.remove(&peer_id);
                            },
                            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                                println!("📉 DISCONNECTED: {} (Cause: {:?})", peer_id, cause);
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

// ENDPOINT 1: Submit Job (Async Return)
async fn submit_job(State(state): State<Arc<AppState>>, Json(payload): Json<ShardedDeployRequest>) -> (StatusCode, Json<JobSubmitResponse>) {
    let task_id = Uuid::new_v4();
    let all_peers = state.stats.lock().await.peers.clone();
    let active_peers: Vec<_> = all_peers.iter()
        .filter(|p| *state.health_registry.get(p).as_deref().unwrap_or(&0) < 2)
        .collect();
    let peer_count = active_peers.len().max(1); 
    
    // Initialize Job State
    let job_state = JobState {
        expected_shards: peer_count,
        results: Vec::new(),
        created_at: Instant::now(),
    };
    state.jobs.insert(task_id, Arc::new(Mutex::new(job_state)));
    
    // Dispatch
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

    println!("📥 Job Submitted: {}", task_id);
    
    (StatusCode::ACCEPTED, Json(JobSubmitResponse {
        job_id: task_id.to_string(),
        status: "pending".to_string()
    }))
}

// ENDPOINT 2: Poll Job Status
async fn get_job_status(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> (StatusCode, Json<JobStatusResponse>) {
    if let Some(job_ref) = state.jobs.get(&id) {
        let guard = job_ref.lock().await;
        let is_complete = guard.results.len() >= guard.expected_shards;
        let is_timeout = guard.created_at.elapsed() > Duration::from_secs(5);

        let sum: i32 = guard.results.iter().map(|(_, val)| val).sum();
        let received_indices: Vec<u32> = guard.results.iter().map(|(idx, _)| *idx).collect();
        let missing_indices: Vec<u32> = (0..guard.expected_shards as u32).filter(|i| !received_indices.contains(i)).collect();

        if is_complete {
            (StatusCode::OK, Json(JobStatusResponse {
                status: "completed".to_string(),
                total_sum: sum,
                breakdown: guard.results.clone(),
                missing_shards: vec![]
            }))
        } else if is_timeout {
            (StatusCode::PARTIAL_CONTENT, Json(JobStatusResponse {
                status: "partial".to_string(),
                total_sum: sum,
                breakdown: guard.results.clone(),
                missing_shards: missing_indices
            }))
        } else {
            // Still waiting
            (StatusCode::OK, Json(JobStatusResponse {
                status: "pending".to_string(),
                total_sum: sum,
                breakdown: guard.results.clone(),
                missing_shards: missing_indices
            }))
        }
    } else {
        (StatusCode::NOT_FOUND, Json(JobStatusResponse {
            status: "not_found".to_string(),
            total_sum: 0,
            breakdown: vec![],
            missing_shards: vec![]
        }))
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
            <div style="display:flex; justify-content:space-between; align-items:center;"><h1>Swarm Cluster</h1><div style="color:var(--green); font-size:11px; border:1px solid var(--green); padding:2px 8px; border-radius:4px; font-weight:bold;">v0.12.0</div></div>
            <div style="color:#94a3b8; font-size:12px; margin-bottom:25px;">Headless API Phase</div>
            <div class="card blue"><div style="color:#94a3b8; font-size:11px; text-transform:uppercase;">Active Peers</div><div style="font-size:28px; font-weight:700;">{}</div></div>
            <div class="card">
                <div style="font-weight:bold; margin-bottom:10px;">🚀 Deploy Wasm</div>
                <input type="file" id="file_in" class="std-input">
                <button id="broadcast-btn" class="btn" onclick="startBroadcast()">SUBMIT JOB (API)</button>
            </div>
            <div class="card" id="res-card" style="display:none;">
                <div id="job-id-text" style="font-size:10px; font-family:monospace; color:#94a3b8; margin-bottom:5px;"></div>
                <div id="status-text" style="font-weight:bold; color:var(--accent);">⏳ Job Pending...</div>
                <div style="font-size:24px; font-weight:bold; margin:10px 0;" id="total-sum"></div>
                <table id="res-table"><thead><tr><th>Shard ID</th><th>Result</th></tr></thead><tbody id="res-body"></tbody></table>
            </div>
            <div class="card" style="padding:15px;"><div style="color:#94a3b8; font-size:11px; text-transform:uppercase;">Live Peer Map</div>{}</div>
        </div>
        <script>
            let b64 = "";
            document.getElementById('file_in').addEventListener('change', function(e) {{ const file = e.target.files[0]; if (!file) return; const reader = new FileReader(); reader.onload = function(ev) {{ b64 = ev.target.result.split(',')[1]; }}; reader.readAsDataURL(file); }});
            
            async function startBroadcast() {{
                if(!b64) return alert("Select a file first!");
                const btn = document.getElementById('broadcast-btn');
                const resCard = document.getElementById('res-card');
                const statusText = document.getElementById('status-text');
                const idText = document.getElementById('job-id-text');
                
                btn.disabled = true; btn.innerText = "Submitting...";
                resCard.style.display = 'block';
                statusText.innerText = "📡 Transmitting to Mesh...";
                statusText.style.color = "var(--accent)";
                
                try {{
                    // 1. Submit Job (Instant Return)
                    const res = await fetch('/api/v1/jobs', {{ method: 'POST', headers: {{ 'Content-Type': 'application/json' }}, body: JSON.stringify({{ wasm_base64: b64, range_size: 100 }}) }});
                    if (!res.ok) throw new Error("Server Error: " + res.status);
                    const data = await res.json();
                    const jobId = data.job_id;
                    
                    idText.innerText = "Job ID: " + jobId;
                    statusText.innerText = "⏳ Computing (Polling API)...";
                    btn.innerText = "Running...";
                    
                    // 2. Poll Status every 500ms
                    const poll = setInterval(async () => {{
                        const statRes = await fetch(`/api/v1/jobs/${{jobId}}`);
                        const statData = await statRes.json();
                        
                        document.getElementById('total-sum').innerText = "Total: " + statData.total_sum;
                        const tbody = document.getElementById('res-body'); tbody.innerHTML = "";
                        statData.breakdown.forEach(row => {{ const tr = document.createElement('tr'); tr.innerHTML = `<td>Shard ${{row[0]}}</td><td style="font-family:monospace; color:#38bdf8;">${{row[1]}}</td>`; tbody.appendChild(tr); }});
                        
                        if (statData.status === 'completed' || statData.status === 'partial') {{
                            clearInterval(poll);
                            btn.disabled = false; btn.innerText = "SUBMIT JOB (API)";
                            if (statData.status === 'completed') {{
                                statusText.innerText = "✅ Job Completed"; statusText.style.color = "var(--green)";
                            }} else {{
                                statusText.innerText = "⚠️ Partial Result (Timeout)"; statusText.style.color = "var(--warn)";
                            }}
                        }}
                    }}, 500);
                    
                }} catch (e) {{ 
                    alert("Submit Failed: " + e); 
                    statusText.innerText = "❌ Error: " + e; statusText.style.color = "#ef4444"; 
                    btn.disabled = false; btn.innerText = "SUBMIT JOB (API)";
                }} 
            }}
            setInterval(() => {{ if(!b64 && document.getElementById('file_in').value === "") location.reload(); }}, 5000);
        </script></body></html>"##, s.peers_count, peer_map
    );
    Html(html)
}
