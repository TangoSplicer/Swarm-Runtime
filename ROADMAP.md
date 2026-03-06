# Swarm Runtime: Roadmap to V1.0 🚀

## Phase 1 to 8: Core Engine, BFT, & State Sync - ✅ COMPLETED
- Fault Tolerance, Async Axum API, Hash-Based Consensus (SHA-256).
- Polyglot edge execution with `wasmi` and `cap-std` sandboxing.
- Persistent Stateful Actors & Asynchronous `tokio::sync::Mutex` locks.
- P2P Pre-Flight State Synchronization & Atomic Smart Contract Routing.

## Phase 9: The Global Mesh & Cryptographic Identity - 🚧 NEXT
- **Unique Peer Identities:** Replace the hardcoded `GATEWAY_SECRET_SEED` with dynamically generated `.swarm_identity` files for every node.
- **Public Cloud Gateway:** Deploy the `swarm-node` Gateway to a public Oracle Cloud VPS (Always Free Tier) to provide a permanent anchor for the Kademlia DHT.
- **NAT Traversal & Mobile Mesh:** Connect Termux mobile edge workers to the public Gateway over cellular networks to prove real-world distributed BFT consensus.

## Phase 10: Swarm Hardening (Reliability) - ⏳ PLANNED
- **Memory Limits & Pruning:** Implement LRU caches and TTL sweeps for `DashMap` storage to prevent Out-Of-Memory (OOM) crashes on edge nodes.
- **Asynchronous Backpressure:** Transition to bounded `tokio::sync::mpsc::channel` queues to gracefully reject traffic floods.
- **Network Resilience:** Implement strict network timeouts and retry logic to handle cellular packet loss.

## Phase 11: Enterprise Federation (Production Readiness) - ⏳ PLANNED
- **Gateway Replication:** Eliminate the single point of failure by allowing backup Gateways to instantly take over scheduling if the primary goes offline.
- **WASM Compilation Caching:** Cache compiled native machine code locally so subsequent smart contract calls execute instantly.
- **Observability:** Add a `/metrics` endpoint to export CPU, memory, and mesh telemetry for open-source dashboards.
- **Polyglot State Sync (WASI VMFS):** Expand the state synchronization engine to support complex sandboxed file transfers (like `sqlite.db`) across the mesh.
