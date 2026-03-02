import os

path = "components/swarm-node/src/worker.rs"
with open(path, "r") as f:
    code = f.read()

target = "match judge.execute(&wasm, &shard_data.data) {"
replacement = "match judge.execute(&wasm, &shard_data.data, &polyglot_id) {"

if target in code:
    code = code.replace(target, replacement)
    with open(path, "w") as f:
        f.write(code)
    print("✅ Worker execute() arguments successfully aligned!")
else:
    print("⚠️ Target not found. It might have different spacing.")
