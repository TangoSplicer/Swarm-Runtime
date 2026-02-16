**Distributed Compute over Libp2p on Mobile.**

Swarm Runtime turns a mesh of Android devices (via Termux) into a distributed supercomputer. It shards WebAssembly tasks, distributes them to peers, and aggregates the results in real-time.

## 🚀 Features
* **Mobile-First:** Runs natively on Android/Termux.
* **Wasm Sandbox:** Safely execute untrusted code on workers.
* **Self-Healing:** (Planned v0.10) Handles node failures automatically.
* **Zero-Config:** Nodes auto-discover via GossipSub.

## 📦 Quick Start (Termux)
1.  **Start Gateway:**
    ```bash
    cargo run -p swarm-node -- gateway
    ```
2.  **Start Workers (New Terminals):**
    ```bash
    cargo run -p swarm-node -- start --shard 1
    cargo run -p swarm-node -- start --shard 2
    ```
3.  **Dashboard:** Open `http://localhost:3000`
4.  **Deploy:** Upload `valid_sum.wasm` and watch the swarm calculate.

## 🛠️ Status
* **v0.9.3 (Current):** Stable Wasm loader, Live Dashboard, 2-way Telemetry.
* **v0.10.0 (Next):** Fault Tolerance.

