# Swarm Runtime: Alignment Protocol

## 1. Core Mandates
* **No Hallucinations:** Verify all crates (axum 0.6, libp2p 0.51, wasmi 0.31, tokio 1.x) before suggesting code.
* **Termux Native:** All Edge Workers must run on Android via Termux. No Docker.
* **Cloud Gateways:** Orchestration Gateways must run on accessible public IPs to serve as permanent DHT anchors.

## 2. Project State (v1.0.0 Enterprise Federation)
* **Architecture:** Active-Active Global Public Mesh with `Lazarus` Fault Tolerance.
* **Consensus:** Hybrid CRDT (Conflict-free Replicated Data Type) State Merging with BFT verification.
* **Gateways:** Federated Orchestration Gateways sync BFT state hashes seamlessly via `swarm-gateway-sync` Gossipsub.

## 3. The Tokio Async Laws
* **No Blocking Clients:** Synchronous blocking clients (e.g., `reqwest::blocking`) are strictly prohibited to prevent runtime thread abortion panics. Use `tokio::fs` for file I/O.
* **The Clone and Release Law:** Never hold a `DashMap` or `Mutex` lock across an `await` point or inside the Libp2p event loop. 
* **Asynchronous Fault Tolerance:** Component monitoring must be handled via `tokio::spawn` and `mpsc` channels (The Lazarus Pattern).
