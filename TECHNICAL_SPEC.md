# Swarm Runtime: Technical Specification v0.20.0

## 1. System Architecture
* **Topology:** Physical Mesh (Libp2p), Logical Star (Gateway-Coordinator).
* **Control Plane:** `gossipsub` (Used strictly for `TEL:` hardware heartbeats).
* **Data Plane:** `libp2p::request_response` (1-to-1 Unicast TCP streams).
* **Storage Plane:** `libp2p::kad` (Kademlia DHT for VMFS file pinning).
* **Security Layer:** Ed25519 Signatures, 60s TTL, Gas Metering traps.
* **Consensus:** Redundancy Factor 2 + SHA-256 Output State Hashing.

## 2. Core Components
### A. The Gateway (Orchestrator) - `gateway.rs`
* **Role:**
    1. Accepts Wasm via HTTP POST (Lazy Assignment Queue).
    2. Calculates Fitness Scores based on hardware telemetry.
    3. Dispatches payload shards via TCP Unicast to 2 separate peers.
    4. Evaluates Hash-Based Consensus and drops poisoned data.

### B. The Worker (Shard) - `worker.rs`
* **Role:**
    1. Broadcasts hardware telemetry every 10 seconds.
    2. Verifies Ed25519 signatures.
    3. Instantiates Wasmi interpreter with fuel limits and `cap-std` ambient authority.
    4. Injects UTF-8 Strings into 1MB Memory Offset.
    5. Sweeps VMFS sandbox, calculates SHA-256 Hashes of outputs, and pins to DHT.

## 3. Data Protocols
* **Deploy Request:** `ShardedDeployRequest { wasm_base64, dataset: Vec<String> }`
* **Shard Result:** `ShardResult { job_id, shard_index, result, result_hash }`
* **Telemetry Payload:** `Telemetry { peer_id, cpu_usage, free_ram_mb }`
