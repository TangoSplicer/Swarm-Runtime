# Swarm Runtime 🐝
**Mobile-First Distributed Compute Mesh over Libp2p.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Termux](https://img.shields.io/badge/Platform-Android%20%2F%20Termux-green)](https://termux.dev/)
[![Status](https://img.shields.io/badge/Release-v0.16.1%20(Smart%20Scheduler)-blue)](https://github.com/TangoSplicer/Swarm-Runtime)

Swarm Runtime turns Android devices into a fault-tolerant compute cluster. It uses **Libp2p** for mesh networking, **Axum** for a Headless REST API, and **Wasmer** for sandboxed code execution.

## 🚀 New in v0.16.1 (Scheduler & Security Phase)
* **🧠 Mobile-Aware Scheduler:** Gateway dynamically weighs CPU/RAM telemetry to route heavy Wasm payloads to the healthiest phones.
* **🔒 Cryptographic Security:** Ed25519 digital signatures and TTL timestamps prevent malicious actors and replay attacks. 
* **⚡ Unicast Data Plane:** 1-to-1 TCP streams bypass GossipSub broadcast storms, saving massive amounts of mobile bandwidth.
* **🧹 Garbage Collection:** Advanced state pruning prevents Out-Of-Memory (OOM) kills on Android/Termux.

## 📦 Quick Start
*(Prerequisites and Startup commands remain the same as previous versions).*

### 🛠️ Architecture: The Telemetry-Driven Pipeline
* **Queue:** Client submits dataset to Gateway (Returns `202 Accepted`).
* **Profile:** Workers broadcast `sysinfo` hardware metrics every 10s. 
* **Dispatch:** Gateway executes Weighted Sharding math, cryptographically signs the envelope, and Unicasts directly to chosen workers.
* **Compute:** Workers verify the signature, reply with an instant TCP ACK, and spawn a background tokio thread to execute the Wasmer sandbox.
* **Reduce:** Workers Unicast results back. Gateway aggregates and prunes state.

## 📜 Changelog
All notable changes to this project will be documented in this section.

### [0.16.1] - 2026-02-23
**Added**
* Lazy Assignment API architecture.
* Sysinfo hardware telemetry heartbeats over GossipSub.
* Weighted Sharding math allocation with remainder sweeping.

### [0.16.0] - 2026-02-23
**Added**
* Ed25519 `SignedPayload` envelopes.
* TTL replay attack protection.
* `disconnect_peer_id` active ban hammer for malicious nodes.

### [0.15.0] - 2026-02-23
**Changed**
* Migrated from O(N) GossipSub data routing to 1-to-1 `libp2p::request_response` Unicast streams.
* Implemented "Decoupled ACK" network pattern to solve TCP Half-Open drops.

### [0.14.1] - 2026-02-23
**Added**
* 15-second SLA fault tolerance and peer re-routing.
* 5-minute TTL Garbage Collection sweep to prevent DashMap memory leaks.
