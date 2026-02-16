# Swarm Runtime: Technical Specification v0.9.3

## 1. System Architecture
* **Topology:** Physical Mesh (Libp2p), Logical Star (Gateway-Coordinator).
* **Transport:** TCP/IP via Libp2p (Noise Encryption, Yamux Multiplexing).
* **Consensus:** None (Stateless Compute). Result aggregation happens at Gateway.

## 2. Core Components
### A. The Gateway (Orchestrator)
* **Stack:** Rust, Axum, Tokio.
* **Role:**
    1. Accepts Wasm via **Atomic Broadcast** (Memory-buffered POST).
    2. Shards data range (`u64` range -> `PeerCount`).
    3. Publishes `SHARD` packet to `swarm-shard-1`.
    4. Aggregates `SHARD_RESULT` messages in a `DashMap`.

### B. The Worker (Shard)
* **Stack:** Rust, Wasmer, Libp2p.
* **Role:**
    1. Subscribes to `swarm-shard-1`.
    2. Decodes `SHARD` packet.
    3. **Runtime:** Instantiates Wasmer with strict `(i32, i32) -> i32` signature.
    4. Executes and publishes `SHARD_RESULT`.

## 3. Data Protocols
* **Input:** Multipart/JSON containing Base64 Wasm.
* **Network Packets (GossipSub):**
    * `SHARD:{task_id}:{shard_idx}:{total}:{start}:{end}:{b64_wasm}`
    * `SHARD_RESULT:{task_id}:{shard_idx}:{value}`

## 4. Current Constraints (v0.9.3)
* **Wasm Signature:** Must be `i32` based. `u64` is not supported by the current Judge.
* **Fault Tolerance:** None. If a node dies, the aggregation hangs.
* **Browser Security:** File Inputs must be read on user-gesture (Click), not change event.
