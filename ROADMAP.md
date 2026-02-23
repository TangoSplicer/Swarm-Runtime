# Swarm Runtime: Roadmap to V1.0 🚀

## Phase 1: Robustness (The "Unbreakable" Mesh) - ✅ COMPLETED
- [x] **Fault Tolerance:** 15s SLA Timeout and idle peer re-assignment.
- [x] **Memory Safety:** 5-minute TTL Garbage Collection for completed jobs.
- [x] **Dynamic Discovery:** mDNS Debouncing, Kademlia DHT, Identify protocol.

## Phase 2: Orchestration (The "Smart" Mesh) - ✅ COMPLETED
- [x] **The Headless API:** Async Axum REST API with Lazy Assignment queue.
- [x] **Unicast Data Plane:** Point-to-point `request_response` routing.
- [x] **Cryptographic Security:** Ed25519 Payload Signatures and malicious peer banning.
- [x] **Telemetry Scheduler:** Weighted Sharding based on CPU/RAM metrics.

## Phase 3: System Hardening (The "Production" Mesh) - 🚧 IN PROGRESS
*Focus: Securing the runtime and expanding payload capabilities.*

### v0.17.0: Execution Security & Complex Data
- [ ] **Wasm Gas Metering:** Protect Android CPUs from infinite `while(true)` Wasm loops.
- [ ] **Deterministic Consensus:** Route identical shards to multiple workers to verify math and prevent cheating.
- [ ] **Complex Payload Injection:** Upgrade `Judge` to support passing JSON strings and Structs into linear memory (beyond `i32` arrays).

## Phase 4: Polyglot Ecosystem (The "Docker Killer")
*Focus: Running services, not just functions.*
- [ ] **CLI Tool:** `swarm deploy main.py` -> Auto-wraps Python scripts in Wasm.
- [ ] **WASI-Networking:** Allow Wasm modules to open their own TCP sockets (Service Mode).
