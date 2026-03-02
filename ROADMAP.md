# Swarm Runtime: Roadmap to V1.0 🚀

## Phase 1 & 2: Robustness & Orchestration - ✅ COMPLETED
- [x] Fault Tolerance, SLA Timeouts, Garbage Collection.
- [x] Async Axum REST API, Lazy Assignment, Weighted Sharding.
- [x] 1-to-1 `request_response` Unicast Data Plane.

## Phase 3: System Hardening (The "Production" Mesh) - ✅ COMPLETED
- [x] **Wasm Gas Metering:** 50B gas limit traps for heavy interpreters.
- [x] **Deterministic Consensus:** Redundancy factor 2 for Byzantine Fault Tolerance.
- [x] **Cryptographic Security:** Ed25519 Payload Signatures and peer banning.

## Phase 4: Complex Data & State Transfer (The "Universal" Mesh) - ✅ COMPLETED
*Focus: Breaking free from integer arrays and handling real-world data.*
- [x] **Universal Payloads:** Pass JSON/Strings into WebAssembly Linear Memory.
- [x] **Hash-Based Consensus:** Use SHA-256 for verifying massive data outputs.
- [x] **WASI Integration:** Grant Wasm access to a virtual sandboxed directory using `cap-std`.
- [x] **Virtual Mesh File System (VMFS):** IPFS-style Kademlia DHT file pinning to share Wasm state cross-mesh.

## Phase 5: Polyglot Ecosystem (The "Docker Killer") - ✅ COMPLETED
- [x] **CLI Tool:** `swarm-cli` built for seamless REST API deployment and monitoring.
- [x] **Edge Caching:** Zero-extraction local Wasm caching via `POLYGLOT:` identifiers.
- [x] **Chroot Jail Isolation:** Secure `./rootfs` mapping to prevent Android symlink capability traps.

## Phase 5.5: Ecosystem Expansion - 🚧 IN PROGRESS
*Integrating new runtimes into the Polyglot Cache:*
- [ ] Python (CPython) - ✅ Implemented
- [ ] JavaScript / TypeScript (QuickJS)
- [ ] Go (TinyGo)
- [ ] SQLite
- [ ] Lua
- [ ] Ruby
- [ ] C / C++
- [ ] PHP
- [ ] Zig

## Phase 6: Distributed Data Retrieval
- [ ] **DHT Fetch:** Upgrade Gateway and CLI to stream files pinned in the Kademlia DHT back to the user.

## Phase 7: Persistent Stateful Actors
- [ ] **Smart Contracts:** Implement persistent state trees for long-running Wasm microservices over libp2p.

## Phase 8: Cross-Device Topology
- [ ] **WAN/LAN Routing:** Inject STUN/TURN NAT traversal to break out of localhost.
