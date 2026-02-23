# Swarm Runtime: Technical Specification v0.16.1

## 1. System Architecture
* **Topology:** Physical Mesh (Libp2p), Logical Star (Gateway-Coordinator).
* **Control Plane:** `gossipsub` (Used strictly for `TEL:` hardware heartbeats and Node Commands).
* **Data Plane:** `libp2p::request_response` (1-to-1 Unicast TCP streams to prevent broadcast storms).
* **Security:** Static Ed25519 Root of Trust. Payloads are signed, verified, and time-stamped.

## 2. Core Components
### A. The Gateway (Orchestrator)
* **Stack:** Rust, Axum, Tokio, Ed25519-Dalek, DashMap.
* **Role:**
    1. Accepts Wasm via HTTP POST (Lazy Assignment Queue).
    2. Aggregates Worker telemetry (CPU/RAM) via GossipSub.
    3. Calculates Fitness Scores and applies Weighted Sharding math.
    4. Cryptographically signs the payload with a static Ed25519 key.
    5. Dispatches `DispatchShard` via direct Unicast streams.
    6. Monitors 15-second SLAs and 5-minute Garbage Collection TTLs.

### B. The Worker (Shard)
* **Stack:** Rust, Wasmer, Libp2p, Sysinfo.
* **Role:**
    1. Broadcasts hardware telemetry every 10 seconds.
    2. Receives Unicast requests and instantly replies with a decoupled `ACK`.
    3. Verifies Ed25519 payload signatures and TTLs (Bans malicious peers).
    4. **Runtime:** Instantiates Wasmer, injects array into linear memory, executes.
    5. Unicasts `SubmitResult` back to the Gateway.

## 3. Data Protocols
* **Security Envelope:** `SignedPayload { payload_json, expires_at, signature }`
* **Telemetry Payload:** `Telemetry { peer_id, cpu_usage, free_ram_mb }`

## 4. Current Constraints (v0.16.1)
* **Wasm Signature:** Must be `i32` based arrays. Complex structs/strings not yet supported.
* **Android Sysinfo:** Modern Android may restrict CPU usage stats. Scheduler falls back to RAM-only weighting if CPU returns `0.0`.
