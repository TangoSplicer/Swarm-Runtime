# Swarm Runtime (v0.9.3) 🐝
**Distributed Compute over Libp2p on Mobile.**

Turns Android devices into a compute mesh.

## 🚀 Status: v0.9.3 (Stable)
* **Discovery:** Auto-gossip.
* **Compute:** Wasm Sandboxed (Wasmer).
* **UI:** Real-time dashboard with heartbeats and result aggregation.

## 📦 Quick Start
1. `cargo run -p swarm-node -- gateway`
2. `cargo run -p swarm-node -- start --shard 1`
3. Open `localhost:3000` -> Upload `final_sum.wasm` -> Broadcast.
