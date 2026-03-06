# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.23.0%20(Stateful%20Smart%20Contracts)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and the **Wasmi** interpreter for secure, ARM64-safe sandboxed code execution.

## 🚀 New in v0.23.0 (Stateful Smart Contracts & P2P Sync)
* **🧠 Persistent Stateful Actors:** WebAssembly linear memory is now dynamically extracted and injected across executions. Smart contracts can preserve their state (`.state` files) indefinitely.
* **🔗 P2P Pre-Flight State Synchronization:** New workers joining the mesh automatically detect if their local contract state is empty or out-of-date. They dynamically query the Kademlia DHT and fetch the latest 1MB memory snapshot from peers *before* executing a new job.
* **⚖️ Atomic Routing & Deduplication:** The Gateway dynamically distinguishes between stateless MapReduce jobs and Stateful Smart Contracts, routing atomic payloads to ensure deterministic execution, while workers utilize highly concurrent `DashSet` caching to prevent redundant double-claiming.
* **📥 Kademlia DHT File Fetching:** Complete closed-loop data retrieval for both state files and standard `output.txt` artifacts.

## 📦 Architecture: The Polyglot & Compiled Pipeline
1. **Queue:** Client submits code via CLI.
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s via Gossipsub. 
3. **Dispatch & Sync:** Gateway attaches the latest known state hash to the payload. Workers verify their local state matches, downloading peer states if necessary.
4. **Compute:** Workers boot the WASI environment, inject the previous memory state, execute the code, and hash the resulting state.
5. **Consensus:** Gateway enforces Byzantine Fault Tolerance (BFT), requiring Redundant Hashes to match exactly.
