# Swarm Runtime: Technical Specification v0.17.1

## 1. System Architecture
* **Topology:** Physical Mesh (Libp2p), Logical Star (Gateway-Coordinator).
* **Control Plane:** `gossipsub` (Used strictly for `TEL:` hardware heartbeats).
* **Data Plane:** `libp2p::request_response` (1-to-1 Unicast TCP streams).
* **Security Layer:** Static Ed25519 Root of Trust. Payloads are signed and time-stamped.
* **Consensus:** Redundancy Factor 2 with strict identical integer matching.

## 2. Core Components
### A. The Gateway (Orchestrator)
* **Role:**
    1. Accepts Wasm via HTTP POST (Lazy Assignment Queue).
    2. Aggregates Worker telemetry (CPU/RAM).
    3. Calculates Fitness Scores and applies Weighted Sharding math.
    4. Cryptographically signs payloads and dispatches via Unicast to 2 separate peers.
    5. Evaluates Deterministic Consensus and drops poisoned/conflicting data.

### B. The Worker (Shard)
* **Role:**
    1. Broadcasts hardware telemetry every 10 seconds.
    2. Verifies Ed25519 payload signatures and TTLs.
    3. **Runtime:** Instantiates Wasmer `Singlepass` compiler with a `5,000,000` gas limit.
    4. Injects array into linear memory, executes, and Unicasts result back.

## 3. Data Protocols
* **Security Envelope:** `SignedPayload { payload_json, expires_at, signature }`
* **Telemetry Payload:** `Telemetry { peer_id, cpu_usage, free_ram_mb }`
