import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# 1. Update the dashboard with perfect v0.6.1 styling + unique ID tracking
new_dashboard = """
async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let s = state.stats.lock().await;
    let peer_list: String = s.peers.iter().map(|p| {
        format!("<div class='peer-item'>{}</div>", p)
    }).collect();

    let html = format!(
        "<!DOCTYPE html>
        <html>
        <head>
            <meta charset='UTF-8'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>Swarm Cluster</title>
            <style>
                :root {{ --bg: #0b0f1a; --card: #161b2c; --accent: #3b82f6; --text: #e2e8f0; --dim: #94a3b8; --gateway: #a855f7; }}
                body {{ font-family: sans-serif; background: var(--bg); color: var(--text); margin: 0; padding: 15px; display: flex; justify-content: center; }}
                .app {{ width: 100%; max-width: 450px; }}
                .header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 25px; }}
                .badge {{ color: #10b981; font-size: 11px; border: 1px solid #10b981; padding: 2px 8px; border-radius: 4px; font-weight: bold; }}
                .card {{ background: var(--card); padding: 20px; border-radius: 12px; margin-bottom: 15px; border: 1px solid #232a3d; position: relative; overflow: hidden; }}
                .card::before {{ content: ''; position: absolute; left: 0; top: 0; bottom: 0; width: 4px; }}
                .blue::before {{ background: var(--accent); }}
                .purple::before {{ background: var(--gateway); }}
                .card-label {{ color: var(--dim); font-size: 11px; text-transform: uppercase; letter-spacing: 0.1em; margin-bottom: 5px; }}
                .card-value {{ font-size: 28px; font-weight: 700; color: #fff; }}
                .peer-item {{ background: #0b0f1a; padding: 10px; border-radius: 6px; margin-top: 8px; font-size: 10px; font-family: monospace; border: 1px solid #232a3d; overflow-x: auto; white-space: nowrap; }}
                .btn {{ background: var(--accent); color: white; border: none; width: 100%; padding: 14px; border-radius: 8px; font-weight: bold; margin-top: 15px; cursor: pointer; }}
                .deploy-zone {{ border: 2px dashed #334155; padding: 25px; text-align: center; border-radius: 10px; margin-top: 10px; color: var(--dim); font-size: 14px; }}
                .logs {{ background: #000; color: #4ade80; padding: 15px; border-radius: 8px; font-family: monospace; font-size: 12px; height: 100px; overflow-y: auto; margin-top: 20px; border: 1px solid #232a3d; }}
            </style>
            <script>setTimeout(() => location.reload(), 2000);</script>
        </head>
        <body>
            <div class='app'>
                <div class='header'>
                    <h1 style='font-size: 22px; margin:0;'>Swarm Cluster</h1>
                    <div class='badge'>v0.8.0 RUNNING</div>
                </div>

                <div class='card blue'>
                    <div class='card-label'>Connected Peers</div>
                    <div class='card-value'>{}</div>
                </div>

                <div class='card purple'>
                    <div class='card-label'>Node Role</div>
                    <div class='card-value' style='color: var(--gateway); font-size: 22px;'>GATEWAY</div>
                </div>

                <div class='card'>
                    <div class='card-label'>Deploy Wasm</div>
                    <input type='file' id='wasmFile' style='display:none' onchange='document.getElementById(\"fName\").innerText = this.files[0].name'>
                    <div class='deploy-zone' onclick='document.getElementById(\"wasmFile\").click()' id='fName'>
                        Select .wasm module
                    </div>
                    <button class='btn'>Broadcast to Swarm</button>
                </div>

                <div class='card' style='padding: 15px;'>
                    <div class='card-label'>Live Node Registry</div>
                    {}
                </div>

                <div class='logs'>
                    > [SYSTEM] Swarm v0.8.0 active.<br>
                    > [NETWORK] Probing ports 4001-4010...
                </div>
            </div>
        </body>
        </html>",
        s.peers_count, peer_list
    );
    Html(html)
}
"""

# 2. Aggressive Probing: Increase dialing frequency and range
content = content.replace("4001..4005", "4001..4011")

# 3. Use Regex to swap the dashboard function
import re
content = re.sub(r'async fn dashboard.*?Html\(html\)\n\}', new_dashboard, content, flags=re.DOTALL)

with open(path, "w") as f:
    f.write(content)
