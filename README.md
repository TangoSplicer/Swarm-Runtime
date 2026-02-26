# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.18.1%20(Modular%20Architecture)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and **Wasmer (Singlepass)** for secure sandboxed code execution.

## 🚀 New in v0.18.1 (Universal Data Phase)
* **🧩 Modular Architecture:** Cleanly decoupled Gateway, Worker, and Network routing. 
* **🧬 Universal Payloads:** Pass complex JSON and Strings directly into the WebAssembly sandbox via Host-Managed Linear Memory injection.
* **#️⃣ Hash-Based Consensus:** Uses SHA-256 output state hashing to verify massive deterministic workloads without saturating mobile bandwidth.
* **🛡️ Execution Security:** Gas-metered execution traps prevent infinite loops and protect battery life.

## 📦 Quick Start
*(See SETUP.md for installation instructions).*

### 🛠️ Architecture: The Telemetry & Consensus Pipeline
1. **Queue:** Client submits dataset to Gateway (Returns `202 Accepted`).
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s. 
3. **Dispatch:** Gateway executes Weighted Sharding, duplicates shards for Redundancy, and securely Unicasts them.
4. **Compute:** Workers instantiate Wasm, inject String bytes into linear memory, execute, and calculate a SHA-256 hash.
5. **Consensus:** Gateway waits for Redundant Hashes to match, establishing cryptographic state agreement.
