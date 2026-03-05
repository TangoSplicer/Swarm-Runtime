# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.22.0%20(Data%20Retrieval)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and the **Wasmi** interpreter for secure, ARM64-safe sandboxed code execution.

## 🚀 New in v0.22.0 (Distributed Data Retrieval Phase)
* **📥 Kademlia DHT File Fetching:** Complete closed-loop data retrieval. The CLI can query the decentralized mesh for a specific SHA-256 output hash, and the Gateway will route the request to the correct Worker to stream the raw bytes back to the user.
* **⚡ Fully Asynchronous CLI:** The `swarm-cli` has been upgraded to a pure `tokio` asynchronous runtime, allowing non-blocking HTTP streaming for massive file downloads.
* **🛡️ Hardened BFT Race-Condition Security:** The network strictly enforces Byzantine Fault Tolerance. If two local nodes collide while writing to the same physical disk during testing, the Gateway dynamically detects the resulting Hash Collision and isolates the compromised nodes.
* **📦 Native Zig & Polyglot Engine:** Execute raw `.wasm`, native `.zig`, or interpreted scripts (Python, JS, Lua, Ruby, PHP, SQLite) seamlessly across the edge.

## 📦 Architecture: The Polyglot & Compiled Pipeline
1. **Queue:** Client submits raw code (e.g., `app.rb`) or local source (`test.zig`). The CLI transparently auto-compiles Zig to Wasm locally, or sets an Edge Cache ID.
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s via Gossipsub. 
3. **Dispatch:** Gateway executes Weighted Sharding, duplicates shards for Redundancy, and securely Unicasts them via TCP Libp2p streams.
4. **Compute:** Workers boot the WASI environment, execute the code in a chroot jail, and hash the output state/files.
5. **Consensus:** Gateway waits for Redundant Hashes to match, establishing cryptographic state agreement.
6. **Retrieval:** The user requests the Consensus Hash, and the network streams the resulting file back.
