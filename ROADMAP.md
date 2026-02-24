# Swarm Runtime: Roadmap to V1.0 🚀

## Phase 1: Robustness (The "Unbreakable" Mesh) - ✅ COMPLETED
- [x] **Fault Tolerance:** 15s SLA Timeout and idle peer re-assignment.
- [x] **Memory Safety:** 5-minute TTL Garbage Collection for completed jobs.
- [x] **Dynamic Discovery:** mDNS Debouncing, Kademlia DHT, Identify protocol.

## Phase 2: Orchestration (The "Smart" Mesh) - ✅ COMPLETED
- [x] **The Headless API:** Async Axum REST API with Lazy Assignment queue.
- [x] **Unicast Data Plane:** Point-to-point `request_response` routing.
- [x] **Telemetry Scheduler:** Weighted Sharding based on CPU/RAM metrics.

## Phase 3: System Hardening (The "Production" Mesh) - ✅ COMPLETED
- [x] **Wasm Gas Metering:** Singlepass compiler with 5M gas limit traps to protect Android CPUs.
- [x] **Deterministic Consensus:** Redundancy factor 2 for Byzantine Fault Tolerance.
- [x] **Cryptographic Security:** Ed25519 Payload Signatures and malicious peer banning.

## Phase 4: Complex Data & State Transfer (The "Universal" Mesh) - 🚧 IN PROGRESS
*Focus: Breaking free from integer arrays and handling real-world data.*
- [ ] **Universal Payloads:** Pass JSON/Strings into WebAssembly Linear Memory.
- [ ] **Hash-Based Consensus:** Use SHA-256 for verifying massive data outputs without bandwidth bloat.
- [ ] **WASI Integration:** Virtualized File Systems and OS-level API bridges.

## Phase 5: Polyglot Ecosystem (The "Docker Killer")
- [ ] **CLI Tool:** `swarm deploy main.py` -> Auto-wraps scripts in Wasm.
- [ ] **Persistent Actors:** Long-running Wasm microservices over libp2p.
