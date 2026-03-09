# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.25.0%20(Global%20Mesh%20Ignition)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and the **Wasmi** interpreter for secure, ARM64-safe sandboxed code execution.

## 🔥 New in v0.25.0 (Global Mesh Ignition)
* **Public Cloud Gateway:** Deployed the orchestration Gateway to an Oracle Cloud VPS, proving WAN connectivity.
* **NAT Traversal:** Workers successfully dial across the public internet to connect to the global Kademlia DHT.
* **REST to Gossipsub Pipeline:** Axum API HTTP requests successfully deserialize strict JSON and broadcast tasks to Android edge nodes globally.

## 🏗 Architecture: The Polyglot & Compiled Pipeline
1. **Queue:** Client submits code via REST API POST request.
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s via Gossipsub. 
3. **Dispatch & Sync:** Gateway attaches the latest known state hash to the payload. Workers verify their local state matches, downloading peer states if necessary.
4. **Compute:** Workers boot the WASI environment, inject the previous memory state, execute the code, and hash the resulting state.
5. **Consensus:** Gateway enforces Byzantine Fault Tolerance (BFT), requiring Redundant Hashes to match exactly.
