# Swarm Runtime
**Mobile-first distributed compute mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Status-Hardened%20main-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a REST API, and the **Wasmi** interpreter for ARM64-safe WASI execution. The current `main` branch requires signed job admission and verifies signed worker results before consensus processing.

## 🔥 New in v1.0.0 (Enterprise Federation)
* **Active-Active Gateways:** Cloud Gateways replicate state seamlessly using Gossipsub. If your primary cloud instance crashes, the secondary instance takes over with zero data loss.
* **HA Fallback Routing:** Android Workers and the Dev CLI seamlessly dial secondary gateways if the primary connection drops.
* **Strict Cryptography:** Network bounds hardened with strict Gossipsub signature validation and SHA-256 deterministic sharding.

## 🏗 Architecture: The Polyglot & Compiled Pipeline
1. **Queue:** Client submits code via the `swarm-node deploy` CLI (e.g., `test_payloads/test_python.py`) to any federated Gateway.
2. **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s via Gossipsub. 
3. **Dispatch & Sync:** Gateway attaches the latest known state hash. Workers verify local state matches, downloading peer states via Kademlia DHT if necessary.
4. **Compute:** Workers boot the WASI environment, inject the previous memory state, execute the code, and hash the resulting state.
5. **Consensus:** Gateways enforce Byzantine Fault Tolerance (BFT), requiring redundant execution before broadcasting the final state hash to the Federation Sync Plane.

## Current security and deployment contract

The gateway accepts a job only when the request includes a short-lived **Ed25519 admission token** covering the exact payload, metadata, nonce, and expiry. The gateway rejects replayed, expired, malformed, altered, or unauthorized requests. Workers require a configured trusted gateway key before accepting dispatch and now return signed result envelopes that the gateway verifies before assignment and consensus processing.

Each shard runs in a newly created workspace under `./swarm-workspaces`; the WASI guest receives only that workspace as a preopened directory. The executor limits module size, dataset, state, output, and fuel consumption. Runtime selection is explicit in signed deployment metadata: raw Wasm workloads submit a module, while Python, JavaScript, Lua, Ruby, PHP, and SQLite workloads resolve a reviewed local Wasm runtime from `./runtimes`.

Read [SECURITY_OPERATIONS.md](SECURITY_OPERATIONS.md) before deploying. It covers admission-key creation, gateway startup, signed deployment, local runtime provisioning, limits, and the remaining framework-upgrade work.

## Verification

The current implementation passes `cargo fmt --all -- --check`, strict Clippy with warnings denied, and `cargo test --workspace --all-targets --locked` (**9 tests passed**). Run the same checks locally before accepting dependency updates:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo audit --file Cargo.lock
```
