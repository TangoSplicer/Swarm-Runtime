import re

path = "components/swarm-node/src/worker.rs"
with open(path, "r") as f:
    code = f.read()

# Intercept the Base64 decode step to load local images
target = r'general_purpose::STANDARD\.decode\(&shard_data\.wasm_image\)'
replacement = r'(if shard_data.wasm_image == "POLYGLOT:PYTHON" { Ok(std::fs::read("python.wasm").unwrap_or_default()) } else { general_purpose::STANDARD.decode(&shard_data.wasm_image) })'

new_code = re.sub(target, replacement, code)

with open(path, "w") as f:
    f.write(new_code)
    
print("✅ Worker logic patched: Polyglot Image Cache enabled!")
