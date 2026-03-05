# Swarm Runtime: Technical Specification v0.22.0

## 1. System Architecture
* **Topology:** Physical Mesh (Libp2p), Logical Star (Gateway-Coordinator).
* **Control Plane:** `gossipsub` (Used strictly for `TEL:` hardware heartbeats).
* **Data Plane:** `libp2p::request_response` (1-to-1 Unicast TCP streams) extended with `FetchData` and `DataPayload` custom CBOR protocols.
* **Storage Plane:** `libp2p::kad` (Kademlia DHT for VMFS file pinning).
* **Consensus:** Dynamic Redundancy Factor (Max: 2) + SHA-256 Output State Hashing.

## 2. Distributed Data Retrieval Protocol (Phase 6)
The Gateway acts as an asynchronous bridge between the standard HTTP web (Axum) and the asynchronous P2P mesh (Libp2p).
1. **The Request:** CLI issues an async `GET /api/v1/data/<HASH>`.
2. **The Bridge:** Axum generates a `tokio::sync::oneshot` channel and passes it to the Libp2p event loop via `NodeCommand::FetchFile`.
3. **The Sweep:** The Gateway queries connected peers via `SwarmRequest::FetchData(hash)`.
4. **The VMFS Scan:** The Worker node intercepts the request, hashes the files in its `./rootfs/data` Virtual Mesh File System, and matches the SHA-256 string.
5. **The Stream:** The Worker returns the raw bytes in a `SwarmResponse::DataPayload`. The Gateway pushes the bytes through the oneshot channel, streaming directly back to the CLI.

## 3. Dynamic Polyglot Routing
* **Zero-Extraction Edge Caching:** Massive Wasm engines (Python, Ruby, PHP) are pre-cached on Workers. The CLI sends plain text source with a `POLYGLOT:LANG` routing ID.
* **Local Compilation:** Systems languages (Zig) are compiled to ultra-lean `wasm32-wasi` via the CLI *before* network transmission, bypassing the 2MB Libp2p Stream limit.
