# Swarm Environment Setup

## 1. Termux (Primary)
* **Install:** `pkg install rust git build-essential clang openssl-tool python`
* **Run Gateway:** `cargo run -p swarm-node -- gateway`
* **Run Worker:** `cargo run -p swarm-node -- start --shard 1`

## 2. Cross-Platform
* **Linux/WSL:** Identical to Termux. Ensure `protoc` is installed for libp2p.
* **Windows:** Use WSL2 (Ubuntu). Powershell may have path escaping issues.

## 3. Verification
* **Check Version:** Look for `v0.9.3` in the dashboard header.
* **Check Wasm:** Use `make_final_wasm.py` to generate a valid test file.
