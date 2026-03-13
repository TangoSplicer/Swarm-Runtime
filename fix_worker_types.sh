#!/bin/bash

WORKER_FILE="components/swarm-node/src/worker.rs"

# Fix Error E0599: The match statement needs a string to compare against string literals
sed -i 's/polyglot_id.as_str()/String::from_utf8_lossy(\&polyglot_id).as_ref()/g' "$WORKER_FILE"

# Fix Error E0308: The judge module explicitly expects a &str for the polyglot identifier
sed -i 's/&polyglot_id, previous/String::from_utf8_lossy(\&polyglot_id).as_ref(), previous/g' "$WORKER_FILE"

echo "✅ Worker type mismatches resolved safely."
