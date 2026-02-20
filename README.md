
# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.13.1%20(MapReduce)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and **Wasmer** for sandboxed code execution and linear memory injection.

## 🚀 New in v0.13.1 (MapReduce Phase)
* **🧠 Wasm Linear Memory:** Gateway dynamically injects data arrays directly into the WebAssembly sandbox memory space.
* **⚙️ Headless Orchestration:** Fully asynchronous API (`POST` to submit, `GET` to poll) replaces blocking UI threads.
* **🔀 Data Sharding:** Mathematically slices input datasets based on the active peer count and routes via GossipSub.
* **⚖️ Tie-Breaker Discovery:** P2P connection stability achieved via deterministic Peer ID comparison and 5-second mDNS debouncing.

## 📦 Quick Start

### Prerequisites
* Android Device with **Termux** (or Linux/WSL).
* Rust Toolchain (`rustc`, `cargo`).
* `protoc` (Protocol Buffers compiler).

### 1. Start the Gateway (Orchestrator)
The Gateway manages the Headless API and distributes the MapReduce workloads.
```bash
cargo run -p swarm-node -- gateway

API is active at http://localhost:3000/api/v1/jobs
### 2. Start Workers (Shards)
Run these on separate devices or in separate terminals to join the mesh.
cargo run -p swarm-node -- start --shard 1
cargo run -p swarm-node -- start --shard 2

### 3. Deploy a Distributed Task
Submit a WebAssembly binary and a dataset via the REST API:
curl -X POST http://localhost:3000/api/v1/jobs \
  -H "Content-Type: application/json" \
  -d '{
    "wasm_base64": "AGFzbQEAAAABBwFgAn9/AX8DAgEABQMBAAEHFAIGbWVtb3J5AgAHZXhlY3V0ZQAACi4BLAECfwJAA0AgAyABTw0BIAIgACADQQRsaigCAGohAiADQQFqIQMMAAsLIAILADIEbmFtZQIXAQAEAANwdHIBA2xlbgIDc3VtAwNpZHgDEgEAAgAEZG9uZQEHbXlfbG9vcA==", 
    "dataset": [10, 20, 30, 40]
  }'

The Gateway will immediately return a job_id (e.g., d2062ac9...).
Poll the task to watch the Swarm aggregate the results:
curl http://localhost:3000/api/v1/jobs/<YOUR-JOB-ID>

### 🛠️ Architecture
The Asynchronous MapReduce Pipeline
 * Submit: Client sends a dataset to the Gateway.
 * Map: Gateway slices the dataset into mathematically even chunks based on active mDNS peers and gossips them via Libp2p.
 * Process: Workers receive their specific shard, write the dataset into WebAssembly linear memory, and execute the sandbox.
 * Reduce (Poll): Gateway collects SHARD_RESULT broadcasts. The client polls the API, which returns HTTP 200 OK (Completed), HTTP 206 Partial Content (Timeout), or HTTP 202 Accepted (Pending).


📜 Changelog
All notable changes to this project will be documented in this section. The format is based on Keep a Changelog.
[0.13.1] - 2026-02-20
Added
 * Wasm linear memory byte injection inside Judge to pass dynamic datasets into the sandbox.
Fixed
 * "Greedy Worker" routing bug: Workers now strictly compute only their assigned shard_index.
 * Unused variable compiler warnings across the codebase.
[0.13.0] - 2026-02-20
Changed
 * ShardedDeployRequest JSON payload upgraded to support dynamic integer arrays (dataset).
 * Gateway data routing engine mathematically slices arrays based on the active peer count.
[0.12.0] - 2026-02-20
Added
 * Asynchronous API endpoints (POST /api/v1/jobs and GET /api/v1/jobs/:id).
 * Real-time terminal logging for WebAssembly runtime traps to improve debugging visibility.
Changed
 * Refactored Judge to safely auto-detect Wasm exports regardless of the internal function name.
Removed
 * Synchronous blocking deployment architecture in the Axum web server.
[0.11.0] - 2026-02-20
Added
 * Libp2p Identify protocol to stabilize TCP handshakes and prevent silent protocol drops.
 * 5-second pending_dials DashMap state to prevent mDNS connection flooding.
Fixed
 * TCP simultaneous open race conditions resolved via a deterministic Peer ID alphabetical Tie-Breaker.
🗺️ Roadmap
| Version | Phase | Status | Key Features |
|---|---|---|---|
| v0.10.0 | Robustness | ✅ Done | Timeouts, Health Registry, Idempotency. |
| v0.11.0 | Discovery | ✅ Done | mDNS Debouncing, TCP Tie-Breakers, Identify protocol. |
| v0.12.0 | Headless | ✅ Done | Asynchronous REST API, Wasm sandbox trap logging. |
| v0.13.1 | MapReduce | ✅ Done | Dynamic dataset sharding, Wasm linear memory injection. |
| v0.14.0 | Fault Tolerance | 📅 Planned | GossipSub heartbeats, automatic idle-node reassignment. |
| v0.15.0 | Security | 📅 Planned | Cryptographic signing and validation of Wasm payloads. |
🤝 Contributing
 * Fork the repository.
 * Create a feature branch (git checkout -b feature/amazing-feature).
 * Commit your changes.
 * Push to the branch.
 * Open a Pull Request.


Maintained by TangoSplicer


