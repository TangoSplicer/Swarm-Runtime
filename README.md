# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.20.9%20(Full%20Polyglot)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and the **Wasmi** interpreter for secure, ARM64-safe sandboxed code execution.

## 🚀 New in v0.20.9 (Interpreted Polyglot Mesh)
* **🗣️ 6-Language Ecosystem:** Natively deploy **Python, JavaScript (QuickJS), Lua, Ruby, PHP, and SQLite** directly to the mesh.
* **🛡️ The Hardened Judge:** The WebAssembly execution environment is rigorously protected against ABI mismatches. Thread panics have been eliminated, allowing the Gateway to gracefully handle Wasm execution errors via Cryptographic Consensus.
* **🔄 Dynamic Fault Tolerance:** If a worker drops offline due to hardware failure or execution panic, the Gateway's SLA monitor dynamically scales down the Redundancy Factor to ensure payload completion without hanging.
* **#️⃣ Hash-Based Consensus:** Uses SHA-256 output state hashing to verify massive deterministic workloads and SQLite database mutations without saturating mobile bandwidth.

## 📦 Architecture: The Telemetry & Consensus Pipeline
1. **Queue:** Client submits raw code (e.g., `app.rb`) via `swarm-cli` to Gateway (Returns `202 Accepted`).
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s. 
3. **Dispatch:** Gateway executes Weighted Sharding, duplicates shards for Redundancy, and securely Unicasts them via Libp2p.
4. **Compute:** Workers dynamically load the required WASI Interpreter (e.g., `ruby.wasm`), instantiate the Wasm sandbox, map the VMFS, execute the code, and calculate a SHA-256 hash of the output state/files.
5. **Consensus:** Gateway waits for Redundant Hashes to match, establishing cryptographic state agreement.
