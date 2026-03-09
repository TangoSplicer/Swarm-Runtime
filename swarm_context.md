# Swarm Runtime: Project Context & Architecture

## 1. Project Overview
* **Name:** Swarm Runtime
* **Goal:** A mobile-first, fault-tolerant distributed edge-compute mesh built in Rust.
* **Environment:** Actively developed and tested natively on an Android device using Termux, connecting to an Oracle Cloud VPS Gateway.
* **Current Version:** v0.25.0 (Global Mesh Ignition).

## 2. Core Tech Stack
* **Networking:** rust-libp2p (mDNS for local, explicit Multiaddrs for WAN, GossipSub, Request-Response, Identify, Kademlia DHT).
* **API / Web:** axum and tokio.
* **Execution:** wasmi (Pure-Rust Interpreter, WASI cap-std integration).
* **Hashing:** sha2 (SHA-256 for deterministic consensus).

## 3. Directory Structure
* **swarm-node:** Modular architecture (main.rs, gateway.rs, worker.rs, types.rs). Handles P2P state synchronization and atomic routing.
* **synapse:** Decoupled P2P Networking Layer.
* **judge:** WebAssembly Execution engine with linear memory byte injection, dynamic memory offsets, and VMFS sweeping.
* **swarm-cli:** Developer deployment suite for Base64 Raw Wasm routing.
