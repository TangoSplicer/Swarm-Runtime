#!/bin/bash

# Stage the documentation, cargo manifests, and rust source updates
git add -A

# Commit the version bump and Phase 14 architecture
git commit -m "feat(core): complete Phase 14 Multi-Shard State Merging

- Implemented BTreeMap lexical sorting for deterministic state hashing
- Added CRDT conflict resolution (Commutative addition for numbers, LWW for strings)
- Updated Edge Workers to asynchronously parse and transmit state deltas
- Bumped project version to v0.28.0"

# Create the v0.28.0 Git Tag
git tag -a v0.28.0 -m "Release v0.28.0: Multi-Shard State Merging"

# Push the commits and the tags to the main branch
git push origin main
git push origin v0.28.0

echo "🎉 Successfully tagged and pushed Swarm Runtime v0.28.0 to remote!"
