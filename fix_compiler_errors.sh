#!/bin/bash

# 1. Enable the 'multipart' feature for axum
cd components/swarm-node || exit
cargo add axum --features multipart

# 2. Fix the gateway.rs metadata parsing (Bypassing .text() entirely for type safety)
GATEWAY_FILE="src/gateway.rs"
sed -i 's/if let Ok(text) = field.text().await {/if let Ok(data) = field.bytes().await { let text = String::from_utf8_lossy(\&data);/g' "$GATEWAY_FILE"

# 3. Fix the worker.rs Vec<u8> vs String mismatches
WORKER_FILE="src/worker.rs"

# Clean up unused import
sed -i 's/, Verifier//g' "$WORKER_FILE"

# Swap .find() for byte window scanning
sed -i 's/wasm_b64.find("|STATE:")/wasm_b64.windows(7).position(|w| w == b"|STATE:")/g' "$WORKER_FILE"

# Convert the extracted hash bytes safely back to a String for the logs/structs
sed -i 's/wasm_b64\[idx + 7..\].to_string()/String::from_utf8_lossy(\&wasm_b64[idx + 7..]).to_string()/g' "$WORKER_FILE"

# Slice the raw WebAssembly bytes securely
sed -i 's/wasm_b64 = wasm_b64\[..idx\].to_string();/wasm_b64 = wasm_b64[..idx].to_vec();/g' "$WORKER_FILE"

# Fix the Polyglot string prefix check to use byte literals
sed -i 's/wasm_b64.starts_with("POLYGLOT:")/wasm_b64.starts_with(b"POLYGLOT:")/g' "$WORKER_FILE"

# Match the fallback type to Vec<u8>
sed -i 's/"NONE".to_string()/b"NONE".to_vec()/g' "$WORKER_FILE"

echo "✅ Compiler fixes applied successfully."
