#!/bin/bash

cd components/swarm-node || exit

# 1. Enable the 'json' feature for reqwest
cargo add reqwest --features json

# 2. Fix the string-slice to byte-vector conversion in main.rs
MAIN_FILE="src/main.rs"
sed -i 's/identifier.into_bytes()/identifier.as_bytes().to_vec()/g' "$MAIN_FILE"

echo "✅ CLI errors resolved."
