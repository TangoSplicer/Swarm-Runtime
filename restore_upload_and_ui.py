import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

new_dashboard = """
async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let s = state.stats.lock().await;
    
    let peer_map_items: String = s.peers.iter().enumerate().map(|(i, p)| {
        let port = 4001 + i; 
        format!(
            "<div class='peer-entry'>
                <div class='peer-header'>
                    <span class='node-dot'></span>
                    <span class='peer-id-label'>NODE ID: {}</span>
                </div>
                <div class='peer-addr'>/ip4/127.0.0.1/tcp/{}</div>
                <div class='progress-container'>
                    <div class='progress-text'><span>Shard Progress</span><span id='p-{}'>0%</span></div>
                    <div class='progress-bar-bg'><div id='b-{}' class='progress-bar-fill'></div></div>
                </div>
            </div>", p, port, i, i
        )
    }).collect();

    let html = format!(
        "<!DOCTYPE html>
        <html>
        <head>
            <meta charset='UTF-8'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>Swarm Dashboard</title>
            <style>
                :root {{ --bg: #0b0f1a; --card: #161b2c; --accent: #3b82f6; --text: #e2e8f0; --dim: #94a3b8; --success: #10b981; }}
                body {{ font-family: sans-serif; background: var(--bg); color: var(--text); margin: 0; padding: 15px; display: flex; justify-content: center; }}
                .app {{ width: 100%; max-width: 450px; }}
                .card {{ background: var(--card); padding: 20px; border-radius: 12px; margin-bottom: 15px; border: 1px solid #232a3d; }}
                .card-label {{ color: var(--dim); font-size: 11px; text-transform: uppercase; margin-bottom: 5px; }}
                .card-value {{ font-size: 32px; font-weight: 800; color: #fff; }}
                .peer-entry {{ background: #0b0f1a; padding: 12px; border-radius: 8px; margin-top: 10px; border: 1px solid #232a3d; }}
                .node-dot {{ width: 8px; height: 8px; background: var(--accent); border-radius: 50%; display: inline-block; margin-right: 8px; box-shadow: 0 0 8px var(--accent); }}
                .peer-id-label {{ font-family: monospace; font-size: 10px; color: var(--dim); }}
                .peer-addr {{ font-family: monospace; font-size: 11px; color: var(--success); margin: 4px 0 8px 16px; }}
                .progress-bar-bg {{ width: 100%; height: 4px; background: #232a3d; border-radius: 2px; overflow: hidden; }}
                .progress-bar-fill {{ width: 0%; height: 100%; background: var(--success); transition: width 0.5s; }}
                .progress-text {{ display: flex; justify-content: space-between; font-size: 9px; color: var(--dim); margin-bottom: 4px; }}
                .upload-box {{ border: 2px dashed #334155; padding: 20px; border-radius: 10px; text-align: center; cursor: pointer; color: var(--dim); font-size: 14px; }}
                .btn {{ background: var(--accent); color: white; border: none; width: 100%; padding: 14px; border-radius: 8px; font-weight: bold; margin-top: 15px; cursor: pointer; }}
                .logs {{ background: #000; color: #4ade80; padding: 15px; border-radius: 8px; font-family: monospace; font-size: 11px; height: 80px; overflow-y: auto; margin-top: 20px; }}
            </style>
        </head>
        <body>
            <div class='app'>
                <h1>Swarm Cluster</h1>
                <div class='card'><div class='card-label'>Network Connections</div><div class='card-value'>{}</div></div>
                <div class='card'>
                    <div class='card-label'>Live Peer Map</div>
                    {}
                </div>
                <div class='card'>
                    <div class='card-label'>Broadcast Wasm</div>
                    <input type='file' id='wasmFile' style='display:none' onchange='handleFile(this)'>
                    <div class='upload-box' onclick='document.getElementById(\\"wasmFile\\").click()' id='fileName'>Select .wasm module</div>
                    <button class='btn' onclick='broadcast()'>Broadcast To All Shards</button>
                </div>
                <div class='logs' id='log'>> System active. Probing 4001..4015</div>
            </div>
            <script>
                let b64 = '';
                function handleFile(input) {{ 
                    document.getElementById('fileName').innerText = input.files[0].name;
                    const reader = new FileReader();
                    reader.onload = (e) => b64 = e.target.result.split(',')[1];
                    reader.readAsDataURL(input.files[0]);
                }}
                async function broadcast() {{
                    if(!b64) return alert('Select Wasm first');
                    const log = document.getElementById('log');
                    log.innerHTML += '<br>> Broadcasting shards...';
                    
                    // Animate bars to simulate work
                    for(let i=0; i<3; i++) {{
                        setTimeout(() => {{
                            document.getElementById('b-'+i).style.width = '100%';
                            document.getElementById('p-'+i).innerText = '100%';
                        }}, 500 + (i * 300));
                    }}

                    const res = await fetch('/deploy/shard', {{
                        method: 'POST',
                        headers: {{ 'Content-Type': 'application/json' }},
                        body: JSON.stringify({{ wasm_base64: b64, range_size: 300 }})
                    }});
                    const data = await res.text();
                    log.innerHTML += '<br>> ' + data;
                }}
                // Auto-refresh peers every 5s
                if(!window.location.search.includes('no-refresh')) {{
                    setTimeout(() => location.reload(), 5000);
                }}
            </script>
        </body></html>",
        s.peers_count, peer_map_items
    );
    Html(html)
}
"""

import re
content = re.sub(r"async fn dashboard.*?Html\(html\)\n\}", new_dashboard, content, flags=re.DOTALL)

with open(path, "w") as f:
    f.write(content)
