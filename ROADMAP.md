# Swarm Runtime: Roadmap to V1.0 🚀

## Phase 1 & 2: Robustness & Orchestration - ✅ COMPLETED
- [x] Fault Tolerance, SLA Timeouts, Garbage Collection.
- [x] Async Axum REST API, Lazy Assignment, Weighted Sharding.
- [x] 1-to-1 `request_response` Unicast Data Plane.

## Phase 3: System Hardening (The "Production" Mesh) - ✅ COMPLETED
- [x] **Wasm Gas Metering:** Singlepass compiler with 5M gas limit traps.
- [x] **Deterministic Consensus:** Redundancy factor 2 for Byzantine Fault Tolerance.
- [x] **Cryptographic Security:** Ed25519 Payload Signatures and peer banning.

## Phase 4: Complex Data & State Transfer (The "Universal" Mesh) - 🚧 IN PROGRESS
*Focus: Breaking free from integer arrays and handling real-world data.*
- [x] **Universal Payloads:** Pass JSON/Strings into WebAssembly Linear Memory.
- [x] **Hash-Based Consensus:** Use SHA-256 for verifying massive data outputs.
- [ ] **WASI Integration:** Grant Wasm access to a virtual sandboxed directory.
- [ ] **Virtual Mesh File System (VMFS):** IPFS-style Kademlia DHT file pinning to share Wasm state cross-mesh.

## Phase 5: Polyglot Ecosystem (The "Docker Killer")
- [ ] **CLI Tool:** `swarm deploy app.py` -> Auto-wraps scripts in Wasm interpreters.
- [ ] **Persistent Actors:** Long-running Wasm microservices over libp2p.
