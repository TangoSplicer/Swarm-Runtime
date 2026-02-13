# 🐝 Swarm-Runtime (v0.6.3)

A distributed, stateful WebAssembly compute cluster designed for edge environments. Originally built on **Android (Termux)**, Swarm replaces heavy Docker containers with lightweight **Wasm Sandboxes** that share a P2P state.



> **Status:** v0.6.3 (Stable Alpha). Tested on Android/Termux. Compatible with Linux, macOS, and Windows (untested).

## 🚀 Why Swarm-Runtime?

The modern cloud is too heavy for the edge. Docker requires significant overhead, and Kubernetes is overkill for a handful of mobile or IoT devices. 

Swarm-Runtime provides:
- **Instant Execution:** Wasm modules start in <10ms.
- **Native Persistence:** Sandboxed logic can read/write to the host's `sled` database via secure Host Functions.
- **Zero Configuration P2P:** Auto-discovery via mDNS; no centralized coordinator required.
- **Resource Efficiency:** Runs comfortably on mobile hardware with minimal battery impact.

---

## 📖 Getting Started

### 1. Prerequisites
- **Rust Toolchain:** `cargo` and `rustc` installed.
- **Wasm Target:** `rustup target add wasm32-unknown-unknown`.

### 2. Launch the Swarm
Start a Worker node (Terminal 1):
```bash
cargo run -- start --shard 1

Start the Gateway (Terminal 2):
```bash
cargo run -- gateway --port 8080

### 3. Deploy & Monitor
Open your browser to http://localhost:8080. Use the integrated dashboard to upload .wasm modules and monitor execution logs in real-time.

