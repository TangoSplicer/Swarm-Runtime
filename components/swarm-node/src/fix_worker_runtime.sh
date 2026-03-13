#!/bin/bash

WORKER_FILE="components/swarm-node/src/worker.rs"

# Swap the Base64 decoder for a simple Ok() wrapper returning the raw bytes
sed -i 's/general_purpose::STANDARD.decode(&wasm_b64)/Ok(wasm_b64.clone())/g' "$WORKER_FILE"

echo "✅ Worker runtime Base64 bug resolved."
