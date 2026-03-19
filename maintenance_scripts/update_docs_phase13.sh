#!/bin/bash

echo "🚀 Updating Documentation for Phase 13 Completion and v0.27.0 Bump..."

# 1. Update ROADMAP.md - Mark Phase 13 Complete and update verbs to past tense
sed -i 's/## Phase 13: Core Hardening & Tech Debt Resolution - 🚧 NEXT/## Phase 13: Core Hardening \& Tech Debt Resolution - ✅ COMPLETED/g' docs/ROADMAP.md
sed -i 's/- \*\*Payload & Path Security:\*\* Enforce cryptographic signature/- \*\*Payload \& Path Security:\*\* Enforced cryptographic signature/g' docs/ROADMAP.md
sed -i 's/- \*\*Tokio Deadlock Eradication:\*\* Refactor the Gateway/- \*\*Tokio Deadlock Eradication:\*\* Refactored the Gateway/g' docs/ROADMAP.md
sed -i 's/- \*\*WASI Standardization:\*\* Replace the fragile/- \*\*WASI Standardization:\*\* Replaced the fragile/g' docs/ROADMAP.md
sed -i 's/- \*\*Algorithm & Network Optimization:\*\* Eradicate O(n²)/- \*\*Algorithm \& Network Optimization:\*\* Eradicated O(n²)/g' docs/ROADMAP.md
sed -i 's/- \*\*API & UX Polish:\*\* Connect the Axum/- \*\*API \& UX Polish:\*\* Connected the Axum/g' docs/ROADMAP.md

# Ensure Phase 14 is marked as NEXT
sed -i 's/## Phase 14: Multi-Shard State Merging - ⏳ PLANNED/## Phase 14: Multi-Shard State Merging - 🚧 NEXT/g' docs/ROADMAP.md

# 2. Append Phase 13 Post-Mortems to TECHNICAL.md
cat << 'EOF' >> docs/TECHNICAL.md

## The O(n²) Array Search Bottleneck (Phase 13)
**The Trap:** Iterating through missing execution shards using `.contains()` on a standard `Vec` caused CPU spikes scaling quadratically with the number of shards.
**Solution:** Converted the tracked shard arrays into Rust `HashSet`s, reducing shard verification lookup time to a mathematically guaranteed O(1).

## The WebAssembly Payload Corruption Trap (Phase 13)
**The Trap:** Injecting state into WebAssembly memory by directly appending `|STATE:` byte strings to the end of the binary violated WASM specifications and risked corrupting executable data segments.
**Solution:** Transitioned to the `cap-std` library. State is now securely mounted as a Virtual File System (VMFS) node, allowing WASI-compliant guest modules to read state via standard POSIX file descriptors.
EOF

# 3. Global Version Bump (v0.26.0 -> v0.27.0) across all Markdown files
find docs/ -type f -name "*.md" -exec sed -i 's/v0.26.0/v0.27.0/g' {} +
find . -maxdepth 1 -type f -name "*.md" -exec sed -i 's/v0.26.0/v0.27.0/g' {} +

# 4. Synchronize all Cargo.toml crates to v0.27.0
find . -name "Cargo.toml" -type f -exec sed -i 's/^version = "0.26.0"/version = "0.27.0"/g' {} +

echo "✅ Documentation successfully updated to v0.27.0 (Hardened Core)!"
