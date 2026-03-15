#!/bin/bash

echo "🚀 Updating Root Documentation for v0.26.0 (Enterprise Federation)..."

# ==========================================
# 1. ALIGNMENT.md
# ==========================================
cat << 'EOF' > ALIGNMENT.md
# Swarm Runtime: Alignment Protocol

## 1. Core Mandates
* **No Hallucinations:** Verify all crates (axum 0.6, libp2p 0.51, wasmi 0.31, tokio 1.x) before suggesting code.
* **Termux Native:** All Edge Workers must run on Android via Termux. No Docker.
* **Cloud Gateways:** Orchestration Gateways must run on accessible public IPs to serve as permanent DHT anchors.

## 2. Project State (v0.26.0 Enterprise Federation)
* **Architecture:** Active-Active Global Public Mesh.
* **Gateways:** Federated Orchestration Gateways (e.g., Oracle & AWS) sync BFT state hashes seamlessly via `swarm-gateway-sync` Gossipsub.
* **Workers:** Android mobile nodes with High Availability (HA) routing. If Gateway A fails, Workers instantly dial Gateway B.

## 3. The Tokio Async Laws
* **No Blocking Clients:** Synchronous blocking clients (e.g., `reqwest::blocking`) are strictly prohibited to prevent runtime thread abortion panics. Use `tokio::fs` for file I/O.
* **The Clone and Release Law:** Never hold a `DashMap` lock across an `await` point or inside the Libp2p event loop. Clone the `Arc`, drop the lock, and process via `tokio::spawn`.
EOF

# ==========================================
# 2. CHANGELOG.md
# ==========================================
cat << 'EOF' > CHANGELOG.md
# Swarm Runtime Changelog

## [0.26.0] - 2026-03-15
### Added
- Enterprise Federation: Gateways now operate in Active-Active HA mode, replicating BFT consensus states via the `swarm-gateway-sync` Gossipsub topic.
- High Availability Routing: The Developer CLI and Android Workers now accept multiple, comma-separated Gateway URLs/Multiaddrs. If the primary Gateway fails, they automatically fall back to the secondary.
- Deterministic Sharding: Replaced the non-deterministic `DefaultHasher` in the Prism Compiler with mathematically sound `sha2` (SHA-256) to ensure cross-architecture BFT consensus.
### Changed
- Security Hardening: Locked Libp2p Gossipsub `ValidationMode` to `Strict` to prevent malicious actors from poisoning the control plane with unverified payloads.
- Memory Protection: Slashed the Libp2p `request_response` timeout from 300s to 15s to prevent massive asynchronous RAM leaks during Worker cellular disconnects.

## [0.25.5] - 2026-03-14
### Added
- Networking: Global Mesh Ignition! Successfully deployed the Gateway to an Oracle Cloud VPS (Public WAN).
EOF

# ==========================================
# 3. README.md
# ==========================================
cat << 'EOF' > README.md
# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.26.0%20(Enterprise%20Federated)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and the **Wasmi** interpreter for secure, ARM64-safe sandboxed code execution.

## 🔥 New in v0.26.0 (Enterprise Federation)
* **Active-Active Gateways:** Cloud Gateways replicate state seamlessly using Gossipsub. If your primary cloud instance crashes, the secondary instance takes over with zero data loss.
* **HA Fallback Routing:** Android Workers and the Dev CLI seamlessly dial secondary gateways if the primary connection drops.
* **Strict Cryptography:** Network bounds hardened with strict Gossipsub signature validation and SHA-256 deterministic sharding.

## 🏗 Architecture: The Polyglot & Compiled Pipeline
1. **Queue:** Client submits code via the `swarm-node deploy` CLI to any federated Gateway.
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s via Gossipsub. 
3. **Dispatch & Sync:** Gateway attaches the latest known state hash. Workers verify local state matches, downloading peer states via Kademlia DHT if necessary.
4. **Compute:** Workers boot the WASI environment, inject the previous memory state, execute the code, and hash the resulting state.
5. **Consensus:** Gateways enforce Byzantine Fault Tolerance (BFT), requiring Redundant execution before broadcasting the final state hash to the Federation Sync Plane.
EOF

# ==========================================
# 4. ROADMAP.md
# ==========================================
sed -i 's/## Phase 12: Enterprise Federation (Production Readiness) - ⏳ PLANNED/## Phase 12: Enterprise Federation (Production Readiness) - ✅ COMPLETED/g' ROADMAP.md
sed -i 's/- \*\*Gateway Replication:\*\* Eliminate the single point of failure by allowing backup Gateways to instantly take over scheduling./- \*\*Gateway Replication:\*\* Eliminated the single point of failure by implementing Gossipsub Active-Active `DashMap` state replication.\n- \*\*HA Routing:\*\* Mobile workers auto-dial secondary gateways upon connection drops./g' ROADMAP.md

cat << 'EOF' >> ROADMAP.md

## Phase 13: Multi-Shard State Merging - 🚧 NEXT
- **Cryptographic Merging:** Build the logic to take multiple distributed execution shards and cryptographically merge their output states back together into a single master database.
- **Edge Dashboard:** Build a lightweight HTML/JS dashboard served natively by the Axum Gateway to visually track the active Termux workers and hardware telemetry.
EOF

# ==========================================
# 5. TECHNICAL_SPEC.md
# ==========================================
sed -i 's/Technical Specification v0.25.5/Technical Specification v0.26.0/g' TECHNICAL_SPEC.md
sed -i 's/\* \*\*Control Plane:\*\* `gossipsub` (Used strictly for `TEL:` hardware heartbeats)./\* \*\*Control Plane:\*\* `gossipsub` (Tracks `TEL:` hardware heartbeats and `SYNC_STATE:` Federation BFT replication)./g' TECHNICAL_SPEC.md

# ==========================================
# 6. TECHNICAL.md
# ==========================================
cat << 'EOF' >> TECHNICAL.md

## The Tokio Mutability Trap (Phase 12)
**The Trap:** Attempting to lock the global `DashMap` ledger inside the Libp2p `Swarm` event loop or across an `await` point causes Tokio thread starvation and complete server deadlock.
**Solution:** The "Clone and Release" mandate. The Gateway clones the `Arc<DashMap>`, drops the lock instantly, and offloads JSON replication parsing to a separate `tokio::spawn` task.

## The Permissive Gossip & Infinite Timeout Trap (Phase 12)
**The Trap:** `request_response` channels defaulting to 300s timeouts caused massive memory leaks when Termux workers dropped cellular connections. Furthermore, permissive Gossipsub allowed potential BFT poisoning.
**Solution:** Reduced timeouts strictly to 15s. Locked Gossipsub `ValidationMode` to `Strict` to cryptographically verify every packet in the control plane.
EOF

# ==========================================
# 7. USER_MANUAL.md
# ==========================================
sed -i 's/Developer User Manual (v0.25.5)/Developer User Manual (v0.26.0)/g' USER_MANUAL.md
sed -i 's/\* \*\*Deploy Python:\*\* `cargo run --bin swarm-node -- deploy app.py --lang python --gateway http:\/\/<PUBLIC_IP>:3000`/\* \*\*Deploy Python (Federated):\*\* `cargo run --bin swarm-node -- deploy app.py --lang python --gateways http:\/\/<PRIMARY_IP>:3000,http:\/\/<SECONDARY_IP>:3000`/g' USER_MANUAL.md

# ==========================================
# 8. Minor Version Bumps (TERMUX_ENV & SETUP)
# ==========================================
sed -i 's/v0.25.0/v0.26.0/g' TERMUX_ENV.md
sed -i 's/Global Mesh Testing (Current):/Global Mesh Testing (Federated):/g' SETUP.md

echo "✅ All Root Markdown Documentation updated to v0.26.0!"
