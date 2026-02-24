# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.17.1%20(Execution%20Security)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and **Wasmer (Singlepass)** for secure sandboxed code execution.

## 🚀 New in v0.17.1 (System Hardening Phase)
* **🛡️ Deterministic Consensus:** Defeats malicious nodes by routing identical data to multiple peers and cryptographically verifying the results match.
* **⛽ Gas Metering:** Protects mobile hardware from infinite loops by trapping Wasm execution at strict instruction limits.
* **🧠 Mobile-Aware Scheduler:** Dynamically weighs CPU/RAM telemetry to route heavy Wasm payloads to the healthiest phones.
* **🔒 Cryptographic Security:** Ed25519 digital signatures and TTL timestamps prevent replay attacks.

## 📦 Quick Start
*(See SETUP.md for installation instructions).*

### 🛠️ Architecture: The Telemetry & Consensus Pipeline
1. **Queue:** Client submits dataset to Gateway (Returns `202 Accepted`).
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s. 
3. **Dispatch:** Gateway executes Weighted Sharding, duplicates shards for Redundancy, and securely Unicasts them.
4. **Compute:** Workers verify signatures, instantiate the `Singlepass` gas-metered Wasm sandbox, and execute.
5. **Consensus:** Gateway waits for redundant results to match. If they conflict, malicious nodes are banned.
