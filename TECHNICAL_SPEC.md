# Swarm Runtime: Technical Specification v0.20.9

## 1. System Architecture
* **Topology:** Physical Mesh (Libp2p), Logical Star (Gateway-Coordinator).
* **Control Plane:** `gossipsub` (Used strictly for `TEL:` hardware heartbeats).
* **Data Plane:** `libp2p::request_response` (1-to-1 Unicast TCP streams).
* **Storage Plane:** `libp2p::kad` (Kademlia DHT for VMFS file pinning).
* **Security Layer:** Ed25519 Signatures, 60s TTL, Gas Metering traps (50 Billion Ops).
* **Consensus:** Dynamic Redundancy Factor (Max: 2) + SHA-256 Output State Hashing.

## 2. Dynamic Polyglot Routing
The Swarm uses a "Zero-Extraction" Edge Caching architecture to bypass the Libp2p 2MB payload limit. 
1. The CLI attaches a `POLYGLOT:LANG` identifier to the deployment payload.
2. The Worker intercepts this ID and loads the corresponding pre-cached WASI binary (e.g., `php.wasm`, `qjs.wasm`) from local storage into memory.
3. The `Judge` writes the incoming payload to a language-specific virtual file (`app.php`, `app.js`) in the `/rootfs/data` chroot jail.
4. The `Judge` dynamically constructs the WASI arguments (e.g., `["php", "/data/app.php"]`) and boots the engine.

## 3. Core Components
### A. The Gateway (Orchestrator) - `gateway.rs`
* **Role:**
    1. Accepts Polyglot Code via HTTP POST.
    2. Calculates Fitness Scores based on hardware telemetry.
    3. Dispatches payload shards via TCP Unicast to 2 separate peers.
    4. Evaluates Hash-Based Consensus.
    5. **SLA Monitor:** Monitors peer timeouts. If a peer drops a shard, dynamically downgrades Redundancy Factor to 1 to force completion.

### B. The Worker (Shard) - `worker.rs` & `judge`
* **Role:**
    1. Broadcasts hardware telemetry every 10 seconds.
    2. Maps `./rootfs` strictly via `cap-std` ambient authority (Chroot Jail).
    3. Executes Wasm safely using `.map_err()` to prevent Host thread panics.
    4. Sweeps VMFS sandbox, calculates SHA-256 Hashes of outputs, and pins to DHT.
