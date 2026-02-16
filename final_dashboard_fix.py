import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# We are replacing the dashboard with a RAW STRING version to stop the escape errors
new_dashboard = r##"
async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let s = state.stats.lock().await;
    let peer_items: String = s.peers.iter().enumerate().map(|(i, p)| {
        format!(
            r#"<div style="background: #0b0f1a; padding: 12px; border-radius: 8px; margin-top: 10px; border: 1px solid #232a3d;">
                <div style="font-family: monospace; font-size: 10px; color: #94a3b8;">NODE ID: {}</div>
                <div style="font-family: monospace; font-size: 11px; color: #10b981; margin: 4px 0 8px 0;">/ip4/127.0.0.1/tcp/{}</div>
                <div style="width: 100%; height: 4px; background: #232a3d; border-radius: 2px;"><div id="b-{}" style="width: 0%; height: 100%; background: #10b981; transition: width 0.5s;"></div></div>
            </div>"#, p, 4001+i, i
        )
    }).collect();

    let html = format!(
        r##"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
        <style>
            :root {{ --bg: #0b0f1a; --card: #161b2c; --accent: #3b82f6; --text: #e2e8f0; }}
            body {{ font-family: sans-serif; background: var(--bg); color: var(--text); padding: 15px; display: flex; justify-content: center; }}
            .app {{ width: 100%; max-width: 450px; }}
            h1 {{ font-size: 26px; font-weight: 800; color: #38bdf8; margin: 0; }}
            .card {{ background: var(--card); padding: 20px; border-radius: 12px; margin-bottom: 15px; border: 1px solid #232a3d; position: relative; }}
            .card::before {{ content: ''; position: absolute; left: 0; top: 0; bottom: 0; width: 4px; }}
            .blue::before {{ background: var(--accent); }}
            .purple::before {{ background: #a855f7; }}
            .yellow::before {{ background: #eab308; }}
            .label {{ color: #94a3b8; font-size: 11px; text-transform: uppercase; letter-spacing: 0.1em; margin-bottom: 8px; }}
            .value {{ font-size: 28px; font-weight: 700; color: #fff; }}
            .upload {{ border: 2px dashed #334155; padding: 25px; border-radius: 10px; text-align: center; cursor: pointer; color: #94a3b8; margin-top: 10px; }}
            .btn {{ background: var(--accent); color: white; border: none; width: 100%; padding: 14px; border-radius: 8px; font-weight: bold; margin-top: 15px; cursor: pointer; }}
            .logs {{ background: #000; color: #4ade80; padding: 15px; border-radius: 8px; font-family: monospace; font-size: 12px; height: 120px; overflow-y: auto; margin-top: 20px; border: 1px solid #232a3d; }}
        </style></head>
        <body><div class="app">
            <div style="display:flex; justify-content:space-between; align-items:center;">
                <h1>Swarm Cluster</h1>
                <div style="color: #10b981; font-size: 11px; border: 1px solid #10b981; padding: 2px 8px; border-radius: 4px; font-weight: bold;">v0.8.0 RUNNING</div>
            </div>
            <div style="color: #94a3b8; font-size: 12px; margin-bottom: 25px;">v0.8.0 Compute Runtime</div>
            <div class="card blue"><div class="label">Connected Peers</div><div class="value">{}</div></div>
            <div class="card purple"><div class="label">Node Role</div><div class="value" style="color:#a855f7;">GATEWAY</div></div>
            <div class="card yellow"><div class="label">Active Shards</div><div class="value" style="color:#eab308;">{}</div></div>
            <div class="card">
                <div style="font-weight:bold; margin-bottom:15px;">🚀 Deploy Wasm</div>
                <input type="file" id="wasm" style="display:none">
                <div class="upload" id="f" onclick="document.getElementById('wasm').click()">Select .wasm module</div>
                <button class="btn" onclick="broadcast()">Broadcast to Swarm</button>
            </div>
            <div class="card" style="padding:15px;"><div class="label">Live Peer Map</div>{}</div>
            <div class="logs" id="log">> System ready.</div>
        </div>
        <script>
            let b64Data = '';
            document.getElementById('wasm').addEventListener('change', function(e) {{
                const file = e.target.files[0];
                if (!file) return;
                document.getElementById('f').innerText = file.name;
                const reader = new FileReader();
                reader.onload = ev => b64Data = ev.target.result.split(',')[1];
                reader.readAsDataURL(file);
            }});

            async function broadcast() {{
                if(!b64Data) return alert('Select .wasm first');
                const log = document.getElementById('log');
                log.innerHTML += '<br>> Broadcasting...';
                
                const count = document.querySelectorAll('.progress-fill').length;
                for(let i=0; i<count; i++) {{
                    document.getElementById('b-'+i).style.width = '100%';
                }}

                const res = await fetch('/deploy/shard', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify({{ wasm_base64: b64Data, range_size: 300 }})
                }});
                const text = await res.text();
                log.innerHTML += '<br>> ' + text;
            }}
            setInterval(() => {{ if(!b64Data) location.reload(); }}, 5000);
        </script></body></html>"##,
        s.peers_count, s.peers_count, peer_items
    );
    Html(html)
}
"##

import re
content = re.sub(r'async fn dashboard.*?Html\(html\)\n\}', new_dashboard, content, flags=re.DOTALL)

with open(path, "w") as f:
    f.write(content)
