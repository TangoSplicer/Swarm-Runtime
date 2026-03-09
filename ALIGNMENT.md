# Swarm Runtime: Alignment Protocol

## 1. Core Mandates
* **No Hallucinations:** Verify all crates (axum 0.6, libp2p 0.51, wasmi 0.31, tokio 1.x) before suggesting code.
* **Termux Native:** All Edge Workers must run on Android via Termux. No Docker.
* **Cloud Gateways:** Orchestration Gateways must run on accessible public IPs to serve as permanent DHT anchors.

## 2. Project State (v0.25.0 Gold)
* **Architecture:** Global Public Mesh.
* **Gateway:** Orchestrates execution from a public cloud instance, handles Axum REST requests, tracks Global State Hashes.
* **Workers:** Android mobile nodes that dial the public Gateway, execute `wasmi` runtimes, and auto-sync missing states.

## 3. The Tokio Async Law
* **No Blocking Clients:** Because the architecture relies heavily on `tokio::spawn`, synchronous blocking clients (e.g., `reqwest::blocking`) are strictly prohibited to prevent runtime thread abortion panics. Use `tokio::fs` for file I/O.
