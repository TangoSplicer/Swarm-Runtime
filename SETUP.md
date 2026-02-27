# Swarm Environment Setup

## 1. Termux (Primary Android Environment)
* **Install:** `pkg install rust git build-essential clang openssl-tool python`
* **Crucial Build Flags:** Due to Android's memory limits, you must throttle the compiler:
  ```bash
  export RUST_MIN_STACK=8388608
  CARGO_PROFILE_DEV_DEBUG=0 cargo build --workspace -j 1
​Run Gateway: cargo run --bin swarm-node -- gateway --port 3000
​Run Worker: cargo run --bin swarm-node -- start --shard 1
​2. Cross-Platform
​Linux/WSL: Identical to Termux. Standard cargo build works fine.
​Windows: Use WSL2 (Ubuntu).
​3. Verification
​Check Version: Look for v0.20.0 (Universal VMFS & Engine Pivot) in the dashboard header at http://localhost:3000.
