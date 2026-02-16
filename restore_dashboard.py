import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

dashboard_html = """
async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let s = state.stats.lock().await;
    let peer_list: String = s.peers.iter().map(|p| format!("<li>{}</li>", p)).collect();
    
    let html = format!(
        "<!DOCTYPE html>
        <html lang='en'>
        <head>
            <meta charset='UTF-8'>
            <title>Swarm Dashboard v0.8.0</title>
            <style>
                body {{ font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 40px; }}
                .container {{ max-width: 800px; margin: auto; }}
                .header {{ display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #334155; padding-bottom: 20px; }}
                .status-badge {{ background: #22c55e; color: #fff; padding: 4px 12px; border-radius: 999px; font-size: 0.8rem; text-transform: uppercase; }}
                .grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin-top: 30px; }}
                .card {{ background: #1e293b; padding: 20px; border-radius: 12px; border: 1px solid #334155; }}
                .card h3 {{ margin-top: 0; color: #94a3b8; font-size: 0.9rem; text-transform: uppercase; }}
                .card div {{ font-size: 2rem; font-weight: bold; color: #38bdf8; }}
                .peer-list {{ margin-top: 20px; background: #1e293b; padding: 20px; border-radius: 12px; }}
                ul {{ list-style: none; padding: 0; font-family: monospace; font-size: 0.8rem; color: #94a3b8; }}
                li {{ padding: 8px 0; border-bottom: 1px solid #334155; }}
            </style>
        </head>
        <body>
            <div class='container'>
                <div class='header'>
                    <h1>🐝 Swarm Cluster</h1>
                    <span class='status-badge'>Online</span>
                </div>
                <div class='grid'>
                    <div class='card'><h3>Node Role</h3><div>Orchestrator</div></div>
                    <div class='card'><h3>Active Peers</h3><div>{}</div></div>
                    <div class='card'><h3>Runtime</h3><div>v0.8.0</div></div>
                    <div class='card'><h3>Engine</h3><div>Synapse-Wasm</div></div>
                </div>
                <div class='peer-list'>
                    <h3>Connected Peer IDs</h3>
                    <ul>{}</ul>
                </div>
            </div>
        </body>
        </html>",
        s.peers_count, peer_list
    );
    Html(html)
}
"""

if "use axum::{" in content and "Html" not in content:
    content = content.replace("Router, http::StatusCode", "Router, response::Html, http::StatusCode")

if "fn dashboard" not in content:
    content += dashboard_html

if '.route("/deploy/shard", post(' in content:
    content = content.replace('.route("/deploy/shard", post(', '.route("/", get(dashboard)).route("/deploy/shard", post(')
    content = content.replace("routing::post", "routing::{post, get}")

with open(path, "w") as f:
    f.write(content)
