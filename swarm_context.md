​Swarm Runtime: Project Context & Architecture
​1. Project Overview
​Name: Swarm Runtime
Goal: A mobile-first, fault-tolerant distributed edge-compute mesh built in Rust.
Environment: Actively developed and tested natively on an Android device using Termux.
Current Version: v0.21.1 (Compiled Payload Phase).
​2. Core Tech Stack
​Networking: rust-libp2p (mDNS, GossipSub, Request-Response, Identify, Kademlia DHT).
​API / Web: axum and tokio.
​Execution: wasmi (Pure-Rust Interpreter, WASI cap-std integration).
​Hashing: sha2 (SHA-256 for deterministic consensus).
​3. Directory Structure
​swarm-node: Modular architecture split into main.rs, gateway.rs, worker.rs, and types.rs.
​synapse: Decoupled P2P Networking Layer.
​judge: WebAssembly Execution engine with linear memory byte injection and VMFS sweeping.
​swarm-cli: Developer deployment suite featuring Polyglot engine targeting and Base64 Raw Wasm routing.
