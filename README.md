# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.20.3%20(Polyglot%20Mesh)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and the **Wasmi** interpreter for secure, ARM64-safe sandboxed code execution.

## 🚀 New in v0.20.3 (Polyglot Mesh Phase)
* **🗣️ Polyglot Interpreters:** Run raw Python natively on the mesh using the zero-extraction edge caching architecture.
* **🛠️ Swarm CLI:** A dedicated developer tool (`swarm-cli`) for deploying code and checking cryptographic hash consensus statuses.
* **🛡️ Chroot Jails:** WASI environments are now strictly bound to a virtual `./rootfs`, protecting the underlying Android host OS while ensuring perfect standard library mapping.
* **#️⃣ Hash-Based Consensus:** Uses SHA-256 output state hashing to verify massive deterministic workloads without saturating mobile bandwidth.

## 📦 Quick Start
*(See SETUP.md for installation instructions and User Manual).*
