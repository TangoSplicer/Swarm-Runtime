# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.27.0%20(Enterprise%20Federated)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and the **Wasmi** interpreter for secure, ARM64-safe sandboxed code execution.

## 🔥 New in v0.27.0 (Enterprise Federation)
* **Active-Active Gateways:** Cloud Gateways replicate state seamlessly using Gossipsub. If your primary cloud instance crashes, the secondary instance takes over with zero data loss.
* **HA Fallback Routing:** Android Workers and the Dev CLI seamlessly dial secondary gateways if the primary connection drops.
* **Strict Cryptography:** Network bounds hardened with strict Gossipsub signature validation and SHA-256 deterministic sharding.

## 🏗 Architecture: The Polyglot & Compiled Pipeline
1. **Queue:** Client submits code via the `swarm-node deploy` CLI (e.g., `test_payloads/test_python.py`) to any federated Gateway.
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s via Gossipsub. 
3. **Dispatch & Sync:** Gateway attaches the latest known state hash. Workers verify local state matches, downloading peer states via Kademlia DHT if necessary.
4. **Compute:** Workers boot the WASI environment, inject the previous memory state, execute the code, and hash the resulting state.
5. **Consensus:** Gateways enforce Byzantine Fault Tolerance (BFT), requiring Redundant execution before broadcasting the final state hash to the Federation Sync Plane.
