# Swarm Runtime: Alignment Protocol

## 1. Core Mandates
* **No Hallucinations:** Verify all crates (axum 0.6, libp2p 0.51, wasmi 0.31, tokio 1.x) before suggesting code.
* **Termux Native:** All Edge Workers must run on Android via Termux. No Docker.
* **Cloud Gateways:** Orchestration Gateways must run on accessible public IPs to serve as permanent DHT anchors.

## 2. Project State (v1.0.0 Enterprise Federation)
* **Architecture:** Active-Active Global Public Mesh.
* **Gateways:** Federated Orchestration Gateways (e.g., Oracle & AWS) sync BFT state hashes seamlessly via `swarm-gateway-sync` Gossipsub.
* **Workers:** Android mobile nodes with High Availability (HA) routing. If Gateway A fails, Workers instantly dial Gateway B.

## 3. The Tokio Async Laws
* **No Blocking Clients:** Synchronous blocking clients (e.g., `reqwest::blocking`) are strictly prohibited to prevent runtime thread abortion panics. Use `tokio::fs` for file I/O.
* **The Clone and Release Law:** Never hold a `DashMap` lock across an `await` point or inside the Libp2p event loop. Clone the `Arc`, drop the lock, and process via `tokio::spawn`.
