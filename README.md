# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.10.0%20(Robustness)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for the Gateway API, and **Wasmer** for sandboxed code execution.

## 🚀 New in v0.10.0 (Robustness Phase)
* **🛡️ Fault Tolerance:** Gateway now handles node failures gracefully with a 5-second timeout.
* **⚠️ Partial Results:** Returns `HTTP 206 Partial Content` if some shards fail, rather than hanging indefinitely.
* **🏥 Health Registry:** Automatically detects "Dead Nodes" (Strike System) to prevent future latency penalties.
* **♻️ Idempotency:** Prevents "double-counting" results from gossipsub message duplication.

## 📦 Quick Start

### Prerequisites
* Android Device with **Termux** (or Linux/WSL).
* Rust Toolchain (`rustc`, `cargo`).
* `protoc` (Protocol Buffers compiler).

### 1. Start the Gateway (Orchestrator)
The Gateway hosts the UI and manages task distribution.
```bash
cargo run -p swarm-node -- gateway
# Access Dashboard at http://localhost:3000             ### 2. Start Workers (Shards)
​Run these on separate devices or separate terminals.    cargo run -p swarm-node -- start --shard 1
cargo run -p swarm-node -- start --shard 2
Here is the full bash static block to finalize the v0.10.0 Robustness Release. This script configures your identity, writes the new documentation, snapshots the code, and pushes the tagged release to GitHub.
# 1. Configure Git Identity for this repository
git config user.email "jasonmaoakley@gmail.com"
git config user.name "TangoSplicer"

# 2. Generate the v0.10.0 README
cat > README.md << 'EOF'
# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.10.0%20(Robustness)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for the Gateway API, and **Wasmer** for sandboxed code execution.

## 🚀 New in v0.10.0 (Robustness Phase)
* **🛡️ Fault Tolerance:** Gateway now handles node failures gracefully with a 5-second timeout.
* **⚠️ Partial Results:** Returns `HTTP 206 Partial Content` if some shards fail, rather than hanging indefinitely.
* **🏥 Health Registry:** Automatically detects "Dead Nodes" (Strike System) to prevent future latency penalties.
* **♻️ Idempotency:** Prevents "double-counting" results from gossipsub message duplication.

## 📦 Quick Start

### Prerequisites
* Android Device with **Termux** (or Linux/WSL).
* Rust Toolchain (`rustc`, `cargo`).
* `protoc` (Protocol Buffers compiler).

### 1. Start the Gateway (Orchestrator)
The Gateway hosts the UI and manages task distribution.
```bash
cargo run -p swarm-node -- gateway
# Access Dashboard at http://localhost:3000

2. Start Workers (Shards)
Run these on separate devices or separate terminals.
cargo run -p swarm-node -- start --shard 1
cargo run -p swarm-node -- start --shard 2

3. Deploy a Task
 * Open http://localhost:3000.
 * Upload a .wasm file (must match (i32, i32) -> i32 signature).
 * Click Broadcast.
 * Watch the mesh compute in real-time.
🛠️ Architecture
The "Partial Guard" Pattern
v0.10.0 introduces a non-blocking aggregation loop:
 * Atomic Broadcast: Gateway shards the input range [0..N] and gossips to swarm-shard-1.
 * Bounded Wait: The Gateway waits for results with a 5-second hard timeout.
 * State Recovery:
   * Success: All shards reply → HTTP 200 OK.
   * Partial: Some shards reply → HTTP 206 Partial Content (Result = Sum of available).
   * Failure: No shards reply → HTTP 504 Gateway Timeout.
Tech Stack
 * Transport: Libp2p (TCP/Noise/Yamux) via GossipSub.
 * Runtime: Wasmer (Singlepass compiler).
 * API: Axum (REST/WebSocket).
 * Protocol: Custom JSON over GossipSub (SHARD / SHARD_RESULT).
🗺️ Roadmap
| Version | Phase | Status | Key Features |
|---|---|---|---|
| v0.9.3 | Foundation | ✅ Done | Basic Sharding, Wasm Execution, Dashboard. |
| v0.10.0 | Robustness | 🚀 Live | Timeouts, Health Registry, Idempotency. |
| v0.11.0 | Discovery | 🚧 Todo | Kademlia DHT, Dynamic peer discovery (No hardcoded ports). |
| v0.12.0 | Headless | 📅 Planned | REST API for automated job submission. |
🤝 Contributing
 * Fork the repository.
 * Create a feature branch (git checkout -b feature/amazing-feature).
 * Commit your changes.
 * Push to the branch.
 * Open a Pull Request.
Maintained by TangoSplicer
