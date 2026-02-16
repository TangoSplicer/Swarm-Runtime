import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# 1. Fix the nested quotes in the broadcast button alert
content = content.replace(
    'onclick=\'alert("Deploying sharded WASM compute...")\'',
    'onclick=\'alert(\\\"Deploying sharded WASM compute...\\\")\''
)

# 2. Add the Task Progress Bar to the Peer Map items
# This adds a visual 'Workload' bar to each peer card
progress_html = """
                <div style='margin-top: 8px;'>
                    <div style='display: flex; justify-content: space-between; font-size: 9px; color: var(--dim); margin-bottom: 2px;'>
                        <span>Shard Progress</span>
                        <span>0%</span>
                    </div>
                    <div style='width: 100%; height: 4px; background: #0b0f1a; border-radius: 2px; overflow: hidden;'>
                        <div style='width: 0%; height: 100%; background: var(--success); transition: width 0.3s;'></div>
                    </div>
                </div>
"""

# Injecting the progress bar into the peer-entry div
content = content.replace(
    "</div>\", p, estimated_port",
    progress_html + "            </div>\", p, estimated_port"
)

# 3. Ensure the prober range is sufficient for your 3 shards
content = content.replace("4001..4011", "4001..4015")

with open(path, "w") as f:
    f.write(content)
