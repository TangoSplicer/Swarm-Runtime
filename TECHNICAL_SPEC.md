# Swarm Runtime: Technical Specification v0.23.0

## 1. System Architecture
* **Topology:** Physical Mesh (Libp2p), Logical Star (Gateway-Coordinator).
* **Control Plane:** `gossipsub` (Used strictly for `TEL:` hardware heartbeats).
* **Data Plane:** `libp2p::request_response` (1-to-1 Unicast TCP streams) extended with `FetchData` and `DataPayload` custom CBOR protocols.
* **Storage Plane:** `libp2p::kad` (Kademlia DHT for VMFS file pinning).
* **Consensus:** Dynamic Redundancy Factor (Max: 2) + SHA-256 Output State Hashing.

## 2. Distributed Stateful Smart Contracts (Phase 7 & 8)
Stateful contracts execute sequentially and atomically, bypassing legacy MapReduce slicing.
1. **State Tracking:** The Gateway maintains a global `DashMap` of the latest verified `[contract_hash -> state_hash]`.
2. **Atomic Dispatch:** The Gateway packages the entire payload into a single atomic shard (Shard 0) and dispatches it to multiple workers to satisfy the Redundancy Factor.
3. **Pre-Flight Sync:** The Worker compares its local `.state` hash against the Gateway's expected hash. If out of sync, it issues an internal `FetchData` request to connected peers to download the latest state array.
4. **Memory Injection:** The `judge` module instantiates the `wasmi` environment and explicitly overwrites the linear memory buffer starting at offset `0` with the injected `.state` bytes.
5. **Payload Offset:** To prevent state corruption, dynamic execution payloads are calculated to sit at the absolute end of the available memory buffer (`total_memory - payload_length`), keeping the static state intact.

## 3. Dynamic Polyglot Routing
* **Zero-Extraction Edge Caching:** Massive Wasm engines (Python, Ruby, PHP) are pre-cached on Workers. 
* **Local Compilation:** Systems languages (Zig) are compiled to ultra-lean `wasm32-wasi` via the CLI.
