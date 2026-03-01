import os

path = "components/swarm-node/src/worker.rs"
with open(path, "r") as f:
    code = f.read()

# Strip the unnecessary parentheses
target = r'= (if shard_data.wasm_image == "POLYGLOT:PYTHON" { Ok(std::fs::read("python.wasm").unwrap_or_default()) } else { general_purpose::STANDARD.decode(&shard_data.wasm_image) }) {'
replacement = r'= if shard_data.wasm_image == "POLYGLOT:PYTHON" { Ok(std::fs::read("python.wasm").unwrap_or_default()) } else { general_purpose::STANDARD.decode(&shard_data.wasm_image) } {'

code = code.replace(target, replacement)

with open(path, "w") as f:
    f.write(code)
    
print("✅ worker.rs warning cleaned up.")
