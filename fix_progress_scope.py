import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# 1. Add the missing clone right before the tokio::spawn block
if "let stats_clone = stats.clone();" in content:
    content = content.replace(
        "let stats_clone = stats.clone();",
        "let stats_clone = stats.clone();\n            let progress_clone = progress.clone();"
    )

with open(path, "w") as f:
    f.write(content)
