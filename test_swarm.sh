#!/bin/bash

# Configuration
GATEWAY="http://127.0.0.1:3000"
WASM_PATH="examples/shard-worker/target/wasm32-unknown-unknown/release/shard_worker.wasm"

if [ ! -f "$WASM_PATH" ]; then
    echo "❌ Error: Wasm worker not found. Build it first!"
    exit 1
fi

WASM_B64=$(base64 -w 0 "$WASM_PATH")

run_test() {
    RANGE=$1
    # Expected sum of 0 to (N-1) is (N * (N-1)) / 2
    EXPECTED=$(( (RANGE * (RANGE - 1)) / 2 ))
    
    echo "🧪 Testing Range: $RANGE (Expected Sum: $EXPECTED)"
    
    RESPONSE=$(curl -s -X POST "$GATEWAY/deploy/shard" \
         -H "Content-Type: application/json" \
         -d "{\"wasm_base64\": \"$WASM_B64\", \"range_size\": $RANGE}")
    
    echo "📡 Gateway Response: $RESPONSE"
    
    if [[ "$RESPONSE" == *"$EXPECTED"* ]]; then
        echo "✅ MATCH! Swarm calculated correctly."
    else
        echo "❌ MISMATCH! Check worker logs."
    fi
    echo "---------------------------------------"
}

echo "🐝 Starting Swarm Validation..."
run_test 100
run_test 200
run_test 500
