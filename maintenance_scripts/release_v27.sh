#!/bin/bash

# Stage the documentation and cargo manifest updates
git add -A

# Commit the version bump
git commit -m "docs: complete Phase 13 roadmap and bump project to v0.27.0

- Marked Phase 13 as COMPLETED in ROADMAP.md
- Promoted Phase 14 to NEXT in ROADMAP.md
- Appended WASI and HashSet post-mortems to TECHNICAL.md
- Bumped all markdown documentation and Cargo.toml manifests to v0.27.0"

# Create the v0.27.0 Git Tag
git tag -a v0.27.0 -m "Release v0.27.0: The Hardened Core"

# Push the commits and the tags to the main branch
git push origin main
git push origin v0.27.0

echo "🎉 Successfully tagged and pushed Swarm Runtime v0.27.0 to remote!"
