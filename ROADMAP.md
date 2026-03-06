# Swarm Runtime: Roadmap to V1.0 🚀

## Phase 1 to 6: Robustness, Polyglot, & Data Retrieval - ✅ COMPLETED
- Fault Tolerance, Async Axum API, Hash-Based Consensus (SHA-256).
- Wasm execution via `wasmi` with 50B Gas Limits and `cap-std` chroot jails.
- Edge Cached Polyglot: Python, JS, Lua, Ruby, PHP, SQLite.
- Distributed DHT Fetching over oneshot channels.

## Phase 7: Persistent Stateful Actors - ✅ COMPLETED
- **Smart Contracts:** Persistent state trees via `wasmi` linear memory extraction.
- **Dynamic Memory Offsets:** Safely injected payloads without overwriting static heap data.
- **Thread Safety:** Asynchronous `tokio::sync::Mutex` locks for highly concurrent disk I/O.

## Phase 8: P2P State Synchronization - ✅ COMPLETED
- **Job Deduplication:** `DashSet` caching to prevent workers from computing redundant tasks locally.
- **Atomic Routing:** Gateway bypasses MapReduce logic for deterministic Smart Contract consensus.
- **Pre-Flight Sync:** Late-joining nodes dynamically heal their empty hard drives by fetching `.state` snapshots from connected peers over Libp2p.

## Phase 9: The Final Frontier - 🚧 NEXT
- **Polyglot State Sync (WASI VMFS):** Upgrade the synchronization engine to support transferring complex sandboxed files (like `sqlite.db`) across the mesh, bringing persistent state to Python, PHP, and SQL scripts.
- **Global Deployment:** Move beyond local Termux testing by deploying the Gateway to a cloud VPS and connecting mobile workers via public IPs.
