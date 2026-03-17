#!/bin/bash

# 1. Synchronize all Cargo.toml crates to v0.26.0
find . -name "Cargo.toml" -type f -exec sed -i 's/^version = .*/version = "0.26.0"/g' {} +
echo "✅ Crates synchronized to v0.26.0"

# 2. Stage all modifications (Docs, Patches, Code)
git add .

# 3. Commit the Phase 12 Architecture
git commit -m "Release v0.26.0: Enterprise Federation & Active-Active Gateways

- Implemented HA Fallback Routing in CLI and Workers
- Added swarm-gateway-sync Gossipsub CRDT replication
- Hardened Libp2p timeouts (15s) and Strict Crypto validation
- Patched Prism Compiler to use SHA-256 for deterministic sharding
- Applied Tokio Clone-and-Release pattern to avoid Dashmap deadlocks"

# 4. Create the v0.26.0 Git Tag
git tag -a v0.26.0 -m "Enterprise Federation and BFT Active-Active Mesh"

# 5. Push code and tags to origin (Assumes you have a remote named 'origin')
git push origin main
git push origin v0.26.0

echo "🎉 Successfully tagged and pushed Swarm Runtime v0.26.0 to remote!"
