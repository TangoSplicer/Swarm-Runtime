# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.24.0%20(Public%20Mesh%20Readiness)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and the **Wasmi** interpreter for secure, ARM64-safe sandboxed code execution.

## 🚀 New in v0.24.0 (Public Mesh Readiness)
* **🔑 Cryptographic Identity:** Removed hardcoded seeds. Nodes now generate permanent, unique `ed25519` identities via OS-native `/dev/urandom` and save them to `.swarm_identity`.
* **🌐 Stable Public Peer IDs:** Unified Application and Network layer cryptography. Libp2p now uses the persistent identity, ensuring Gateway and Worker nodes retain the same addressable `PeerId` across cloud server reboots.
* **🔒 Noise Protocol Security:** Payload routing is now implicitly secured and authenticated by the Libp2p `Noise` cryptographic handshake over TCP streams.

## 📦 Architecture: The Polyglot & Compiled Pipeline
1. **Queue:** Client submits code via CLI.
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s via Gossipsub. 
3. **Dispatch & Sync:** Gateway attaches the latest known state hash to the payload. Workers verify their local state matches, downloading peer states if necessary.
4. **Compute:** Workers boot the WASI environment, inject the previous memory state, execute the code, and hash the resulting state.
5. **Consensus:** Gateway enforces Byzantine Fault Tolerance (BFT), requiring Redundant Hashes to match exactly.
