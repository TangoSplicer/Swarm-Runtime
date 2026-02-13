# 🐝 Swarm-Runtime (v0.6.2)
A distributed, stateful WebAssembly compute cluster designed for edge environments (Termux/Mobile/IoT).

## 🚀 The Docker Alternative
| Feature | Docker | Swarm-Runtime |
| :--- | :--- | :--- |
| **Startup Time** | ~500ms - 2s | **< 10ms** |
| **Memory Overhead** | ~50MB+ per container | **~2MB** per sandbox |
| **P2P Mesh** | External | **Built-in** |

## 📖 Quick Start
1. `cargo run -- start --shard 1`
2. `cargo run -- gateway --port 8080`
3. Visit `http://localhost:8080`
