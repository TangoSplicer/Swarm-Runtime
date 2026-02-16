# Swarm Environment Setup

## 1. The "Gold Standard" (Android / Termux)
This is the primary deployment target.
* **Shell:** Bash / Zsh
* **Root:** Not required (Userland).
* **Dependencies:**
    ```bash
    pkg update && pkg upgrade
    pkg install rust git build-essential clang openssl-tool python
    ```
* **Network:** Localhost (127.0.0.1) is used for the swarm mesh.
    * Gateway: `127.0.0.1:3000` (Dashboard)
    * P2P Transport: TCP Ports `4000` (Gateway) and `4001+` (Workers).

## 2. Linux / macOS (Development)
The code is 100% compatible with standard Unix environments.
* **Dependencies:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* **Build:** `cargo build --release` (Same as Termux)

## 3. Windows (Cross-Compat)
* **Recommended:** Use WSL2 (Ubuntu).
* **Native Powershell:**
    * You must install `protoc` (Protocol Buffers) manually for `libp2p`.
    * Path separators in file loading scripts (Python generators) may need adjusting (`\` vs `/`).
    * *Recommendation: Stick to WSL2 to match Termux behavior.*

## 4. Verification Script
Run this to prove your environment is ready:
```bash
cargo run -p swarm-node -- --help
# Should output: Commands: start, gateway

