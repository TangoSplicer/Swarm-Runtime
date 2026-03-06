# Swarm Runtime: Alignment Protocol

## 1. Core Mandates
* **No Hallucinations:** Verify all crates (axum 0.6, libp2p 0.51, wasmi 0.31, tokio 1.x) before suggesting code.
* **Termux Native:** All code must run on Android via Termux. No Docker.
* **Atomic Progress:** Features must be fully implemented and verified before moving to the next version.

## 2. Project State (v0.23.0 Gold)
* **Architecture:** Hub-and-Spoke (Logical) / Mesh (Physical).
* **Gateway:** Orchestrates execution, tracks Global State Hashes, handles BFT consensus, and DHT file retrieval bridging.
* **Workers:** Execute `wasmi` runtimes, inject/extract `.state` memory, auto-sync missing states from peers, and stream VMFS data on demand.
* **CLI:** Multi-route deployment and Asynchronous file downloading.

## 3. The Tokio Async Law
* **No Blocking Clients:** Because the architecture relies heavily on `tokio::spawn` and async network event loops, synchronous blocking clients (e.g., `reqwest::blocking`) are strictly prohibited to prevent runtime thread abortion panics.
