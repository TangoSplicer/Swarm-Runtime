# Swarm Runtime: Roadmap to V1.0 🚀

## Phase 1: Robustness (The "Unbreakable" Mesh)
*Focus: Making v0.9.3 resilient to real-world network chaos.*

### v0.10.0: Fault Tolerance (Self-Healing)
- [ ] **Timeout Detection:** Gateway aborts wait loop if Shards don't reply in 5s.
- [ ] **State Recovery:** Mark unresponsive shards as `DEAD`.
- [ ] **Re-Assignment:** Automatically re-send the `SHARD` payload to a backup peer.

### v0.11.0: Dynamic Discovery (No Hardcoded Ports)
- [ ] **Kademlia DHT:** Replace `gossipsub` looping with `libp2p-kad`.
- [ ] **Bootstrap Nodes:** Allow nodes to join via a single entry point string, not IP:Port.

## Phase 2: Programmability (The "Headless" Era)
*Focus: Allowing Python/JS/Rust scripts to drive the swarm.*

### v0.12.0: The Headless API
- [ ] **REST API:** `POST /api/v1/jobs` (Accepts Wasm bytes).
- [ ] **Job IDs:** Return `uuid` immediately; allow polling `GET /results/{uuid}`.
- [ ] **Removal of Browser Dependency:** The UI becomes just one client of the API.

### v0.13.0: The Job Queue
- [ ] **Concurrency:** Internal Priority Queue for incoming jobs.
- [ ] **Load Balancing:** Round-robin distribution if swarm size > job splits.

## Phase 3: The "Docker Killer" (System Interfaces)
*Focus: Running services, not just functions.*

### v0.14.0: Polyglot SDKs
- [ ] **CLI Tool:** `swarm build main.py` -> Auto-wraps Python in Wasm.
- [ ] **Client Libraries:** Python/JS wrappers for the HTTP API.

### v0.15.0: WASI (Filesystem & Networking)
- [ ] **Virtual FS:** Mount `.zip` payloads as `/app` directories inside Wasm.
- [ ] **WASI-Networking:** Allow Wasm modules to open TCP sockets (Service Mode).

## 🏁 V1.0.0: Production Ready
- [ ] **Security:** API Keys & Resource Quotas (RAM/CPU limits).
- [ ] **Registry:** `swarm pull user/service:latest` (IPFS-backed).
