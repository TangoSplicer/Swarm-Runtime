import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# Fix the nested quotes in the broadcast button alert
content = content.replace(
    'onclick=\'alert("Deploying sharded WASM compute...")\'',
    'onclick=\'alert(\"Deploying sharded WASM compute...\")\''
)

# Ensure the prober range is sufficient for 3+ shards
content = content.replace("4001..4011", "4001..4015")

with open(path, "w") as f:
    f.write(content)
