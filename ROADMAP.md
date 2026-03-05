# Swarm Runtime: Roadmap to V1.0 🚀

## Phase 1 to 5.6: Robustness, Polyglot, and Native Compilation - ✅ COMPLETED
- Fault Tolerance, Async Axum REST API, Hash-Based Consensus (SHA-256).
- Wasm execution via `wasmi` with 50B Gas Limits and `cap-std` chroot jails.
- Edge Cached Polyglot: Python, JS, Lua, Ruby, PHP, SQLite.
- Native Execution: CLI auto-compilation for Zig and base64 `.wasm` routing.

## Phase 6: Distributed Data Retrieval - ✅ COMPLETED
- **DHT Fetch:** Gateway and CLI upgraded to stream `output.txt` files pinned in the Kademlia DHT back to the user via async oneshot channels.

## Phase 7: Persistent Stateful Actors - 🚧 NEXT
- **Smart Contracts:** Implement persistent state trees for long-running Wasm microservices. 
- **Snapshotting:** Dump `wasmi` linear memory to `.state` files and reload them dynamically on subsequent executions to preserve decentralized application memory.
