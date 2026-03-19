#!/bin/bash

echo "🚀 Updating Documentation for Phase 14 Completion and v0.28.0 Bump..."

# 1. Update ROADMAP.md - Mark Phase 14 Complete and Phase 15 as Next
sed -i 's/## Phase 14: Multi-Shard State Merging - 🚧 NEXT/## Phase 14: Multi-Shard State Merging - ✅ COMPLETED/g' docs/ROADMAP.md
sed -i 's/## Phase 15: The Telemetry Control Center - ⏳ PLANNED/## Phase 15: The Telemetry Control Center - 🚧 NEXT/g' docs/ROADMAP.md

# 2. Global Version Bump (v0.27.0 -> v0.28.0) across all Markdown files
find docs/ -type f -name "*.md" -exec sed -i 's/v0.27.0/v0.28.0/g' {} +
find . -maxdepth 1 -type f -name "*.md" -exec sed -i 's/v0.27.0/v0.28.0/g' {} +

# 3. Synchronize all Cargo.toml crates to v0.28.0
find . -name "Cargo.toml" -type f -exec sed -i 's/^version = "0.27.0"/version = "0.28.0"/g' {} +

echo "✅ Documentation successfully updated to v0.28.0 (Multi-Shard State Merging)!"
