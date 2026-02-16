import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    lines = f.readlines()

new_lines = []
skip = False

# Remove the old dashboard function
for line in lines:
    if "async fn dashboard" in line:
        skip = True
    if skip:
        if line.strip() == "}":
            skip = False
        continue
    new_lines.append(line)

# Append the new, high-fidelity dashboard
new_lines.append("""
async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let s = state.stats.lock().await;
    let peer_list: String = s.peers.iter().map(|p| {
        format!("<div style='background: #334155; padding: 10px; border-radius: 5px; margin-bottom: 5px; font-size: 0.8em; border-left: 4px solid #38bdf8;'>Node ID: {}</div>", p)
    }).collect();

    let html = format!(
        "<!DOCTYPE html>
        <html lang='en'>
        <head>
            <meta charset='UTF-8'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>Swarm Command | v0.8.0</title>
            <style>
                :root {{ --bg: #0f172a; --card: #1e293b; --text: #f8fafc; --accent: #38bdf8; --success: #22c55e; }}
                body {{ font-family: 'Inter', sans-serif; background: var(--bg); color: var(--text); margin: 0; padding: 20px; }}
                .container {{ max-width: 1000px; margin: auto; }}
                .navbar {{ display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #334155; padding-bottom: 20px; }}
                .badge {{ background: var(--success); color: white; padding: 4px 12px; border-radius: 20px; font-size: 0.75rem; font-weight: bold; }}
                .metrics {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin-top: 30px; }}
                .metric-card {{ background: var(--card); padding: 20px; border-radius: 12px; border: 1px solid #334155; text-align: center; }}
                .metric-card h3 {{ margin: 0; font-size: 0.8rem; color: #94a3b8; text-transform: uppercase; letter-spacing: 0.05em; }}
                .metric-card div {{ font-size: 2.5rem; font-weight: 800; color: var(--accent); margin-top: 10px; }}
                .content-grid {{ display: grid; grid-template-columns: 1fr 350px; gap: 20px; margin-top: 30px; }}
                .console {{ background: #000; border-radius: 12px; padding: 20px; font-family: 'Fira Code', monospace; font-size: 0.9rem; color: #4ade80; height: 300px; overflow-y: auto; border: 1px solid #334155; }}
                .peer-panel {{ background: var(--card); padding: 20px; border-radius: 12px; border: 1px solid #334155; }}
                .peer-panel h3 {{ margin-top: 0; font-size: 1rem; border-bottom: 1px solid #334155; padding-bottom: 10px; }}
                .cursor {{ display: inline-block; width: 10px; height: 1.2em; background: #4ade80; vertical-align: middle; animation: blink 1s infinite; }}
                @keyframes blink {{ 0%, 100% {{ opacity: 1; }} 50% {{ opacity: 0; }} }}
            </style>
            <script>
                // Refresh every 3 seconds to update peer counts
                setTimeout(() => location.reload(), 3000);
            </script>
        </head>
        <body>
            <div class='container'>
                <div class='navbar'>
                    <h1 style='margin:0;'>🐝 Swarm Cluster</h1>
                    <span class='badge'>v0.8.0 RUNNING</span>
                </div>

                <div class='metrics'>
                    <div class='metric-card'><h3>Active Peers</h3><div>{}</div></div>
                    <div class='metric-card'><h3>Tasks</h3><div>SHARDED</div></div>
                    <div class='metric-card'><h3>Network</h3><div>LOCAL</div></div>
                </div>

                <div class='content-grid'>
                    <div class='console'>
                        <div style='color: #94a3b8;'>// Execution logs</div>
                        <div>> Orchestrator initialized on port 4000...</div>
                        <div>> TCP Prober scanning range [4001-4005]...</div>
                        <div>> GossipSub mesh active for 'swarm-shard-1'...</div>
                        <div style='margin-top: 10px;'>> READY: Awaiting WASM payload via POST /deploy/shard...</div>
                        <div class='cursor'></div>
                    </div>

                    <div class='peer-panel'>
                        <h3>Live Node Registry</h3>
                        <div style='overflow-y: auto; max-height: 230px;'>
                            {}
                        </div>
                    </div>
                </div>
            </div>
        </body>
        </html>",
        s.peers_count, peer_list
    );
    Html(html)
}
""")

with open(path, "w") as f:
    f.writelines(new_lines)
