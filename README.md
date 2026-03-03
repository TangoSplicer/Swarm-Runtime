# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.21.1%20(Compiled%20Payloads)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and the **Wasmi** interpreter for secure, ARM64-safe sandboxed code execution.

## 🚀 New in v0.21.1 (Compiled Payload Phase)
* **⚡ Native Zig Execution:** Compile raw Zig code (`.zig`) directly to microscopic, dependency-free WebAssembly binaries locally on the device, bypassing network limits entirely.
* **📦 Raw Wasm Routing:** Deploy raw `.wasm` binaries encoded in Base64 directly across the mesh.
* **🛡️ Empty Shard Prevention:** The Gateway's Scheduler now strictly assigns pseudo-datasets to dynamically load-balance compiled payloads without falling into the 0-task trap.
* **🗣️ 6-Language Interpreted Ecosystem:** Natively deploy Python, JavaScript (QuickJS), Lua, Ruby, PHP, and SQLite directly to the mesh using edge-cached runtime WASI binaries.
* **🔄 Dynamic Fault Tolerance:** The Gateway's SLA monitor dynamically scales down the Redundancy Factor to ensure payload completion without hanging.

## 📦 Architecture: The Polyglot & Compiled Pipeline
1. **Queue:** Client submits raw code (e.g., `app.rb`) or local source (`test.zig`). The CLI transparently auto-compiles Zig/Go to Wasm locally, or sets an Edge Cache ID.
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s via Gossipsub. 
3. **Dispatch:** Gateway executes Weighted Sharding, duplicates shards for Redundancy, and securely Unicasts them via TCP Libp2p streams.
4. **Compute:** Workers boot the WASI environment (or intercept the local Wasm module natively), execute the code in a chroot jail, and hash the output state/files.
5. **Consensus:** Gateway waits for Redundant Hashes to match, establishing cryptographic state agreement.
