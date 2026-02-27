# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.20.0%20(Universal%20VMFS)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and the **Wasmi** interpreter for secure, ARM64-safe sandboxed code execution.

## 🚀 New in v0.20.0 (Universal VMFS Phase)
* **🧠 Engine Pivot:** Replaced Wasmer JIT with the pure-Rust Wasmi interpreter, permanently resolving ARM64/aarch64 hardware alignment panics on mobile devices.
* **📁 Virtual Mesh File System (VMFS):** WebAssembly tasks can now safely write to a host-mapped `/data` directory using `cap-std` ambient authority.
* **🌐 DHT File Pinning:** Workers sweep the VMFS for output files, hash them, and announce them to the Kademlia DHT for network-wide state sharing.
* **#️⃣ Hash-Based Consensus:** Uses SHA-256 output state hashing to verify massive deterministic workloads without saturating mobile bandwidth.

## 📦 Quick Start
*(See SETUP.md for installation instructions).*

### 🛠️ Architecture: The Telemetry & Consensus Pipeline
1. **Queue:** Client submits dataset to Gateway (Returns `202 Accepted`).
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s. 
3. **Dispatch:** Gateway executes Weighted Sharding, duplicates shards for Redundancy, and securely Unicasts them.
4. **Compute:** Workers instantiate Wasm, inject data, map the VMFS, execute, and calculate a SHA-256 hash of the output state/files.
5. **Consensus:** Gateway waits for Redundant Hashes to match, establishing cryptographic state agreement.
