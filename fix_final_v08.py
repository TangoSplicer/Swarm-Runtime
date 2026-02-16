import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# Update the Prober range to 4001-4010 to be safe
content = content.replace("4001..4005", "4001..4010")

# Replace the dashboard function with the improved UI
new_dashboard = """
async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let s = state.stats.lock().await;
    let peer_list: String = s.peers.iter().map(|p| {
        format!("<div class='peer-item'><code>{}</code></div>", p)
    }).collect();

    let html = format!(
        "<!DOCTYPE html>
        <html>
        <head>
            <meta charset='UTF-8'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>Swarm Cluster v0.8.0</title>
            <style>
                :root {{ --bg: #0b0f1a; --card: #161b2c; --accent: #3b82f6; --text: #e2e8f0; --dim: #94a3b8; }}
                body {{ font-family: -apple-system, blinkmacsystemfont, 'Segoe UI', roboto, sans-serif; background: var(--bg); color: var(--text); margin: 0; padding: 15px; display: flex; justify-content: center; }}
                .app {{ width: 100%; max-width: 500px; }}
                .header {{ display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 20px; }}
                .status-running {{ color: #10b981; font-weight: bold; font-size: 12px; border: 1px solid #10b981; padding: 2px 8px; border-radius: 4px; }}
                .card {{ background: var(--card); padding: 20px; border-radius: 12px; margin-bottom: 15px; border: 1px solid #232a3d; }}
                .card-label {{ color: var(--dim); font-size: 11px; text-transform: uppercase; letter-spacing: 0.1em; margin-bottom: 8px; }}
                .card-value {{ font-size: 28px; font-weight: 700; color: #fff; }}
                .peer-item {{ background: #0b0f1a; padding: 8px 12px; border-radius: 6px; margin-top: 8px; font-size: 10px; border-left: 3px solid var(--accent); overflow-x: auto; white-space: nowrap; }}
                
                /* Deploy Section */
                .deploy-zone {{ border: 2px dashed #334155; padding: 20px; text-align: center; border-radius: 10px; margin-top: 10px; cursor: pointer; }}
                .btn {{ background: var(--accent); color: white; border: none; width: 100%; padding: 12px; border-radius: 8px; font-weight: bold; margin-top: 15px; cursor: pointer; }}
                
                .logs {{ background: #000; color: #4ade80; padding: 15px; border-radius: 8px; font-family: 'Fira Code', monospace; font-size: 12px; height: 120px; overflow-y: auto; margin-top: 20px; border: 1px solid #232a3d; }}
            </style>
        </head>
        <body>
            <div class='app'>
                <div class='header'>
                    <div style='font-size: 24px; font-weight: 800;'>Swarm Cluster</div>
                    <div class='status-running'>v0.8.0 RUNNING</div>
                </div>

                <div class='card'>
                    <div class='card-label'>Connected Peers</div>
                    <div class='card-value'>{}</div>
                </div>

                <div class='card'>
                    <div class='card-label'>Node Role</div>
                    <div style='font-size: 22px; font-weight: bold; color: #a855f7;'>GATEWAY</div>
                </div>

                <div class='card'>
                    <div class='card-label'>Deploy Wasm</div>
                    <input type='file' id='wasmFile' style='display:none' onchange='handleFile()'>
                    <div class='deploy-zone' onclick='document.getElementById(\\"wasmFile\\").click()'>
                        <div id='fileName' style='color: var(--dim)'>Select .wasm module</div>
                    </div>
                    <button class='btn' onclick='broadcast()'>Broadcast to Swarm</button>
                </div>

                <div class='card' style='padding: 15px;'>
                    <div class='card-label'>Live Node Registry</div>
                    {}
                </div>

                <div class='logs' id='log'>
                    > System ready.<br>
                    > Auto-discovery probing ports 4001-4010...
                </div>
            </div>

            <script>
                let b64Data = '';
                function handleFile() {{
                    const file = document.getElementById('wasmFile').files[0];
                    document.getElementById('fileName').innerText = file.name;
                    const reader = new FileReader();
                    reader.onload = (e) => {{ b64Data = e.target.result.split(',')[1]; }};
                    reader.readAsDataURL(file);
                }}

                async function broadcast() {{
                    if(!b64Data) return alert('Select a file first');
                    const log = document.getElementById('log');
                    log.innerHTML += '<br>> Broadcasting to swarm...';
                    
                    const res = await fetch('/deploy/shard', {{
                        method: 'POST',
                        headers: {{ 'Content-Type': 'application/json' }},
                        body: JSON.stringify({{ wasm_base64: b64Data, range_size: 200 }})
                    }});
                    const text = await res.text();
                    log.innerHTML += '<br>> ' + text;
                }}
                
                // Auto-refresh to see peers
                setTimeout(() => location.reload(), 5000);
            </script>
        </body>
        </html>",
        s.peers_count, peer_list
    );
    Html(html)
}
"""

# Surgical replacement of the old dashboard function
import re
content = re.sub(r'async fn dashboard.*?Html\(html\)\n\}', new_dashboard, content, flags=re.DOTALL)

with open(path, "w") as f:
    f.write(content)
