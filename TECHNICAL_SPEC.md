# Swarm Runtime: Technical Specification v0.26.0

## 1. System Architecture
* **Topology:** Physical Mesh (Libp2p), Logical Star (Public Cloud Gateway-Coordinator).
* **Control Plane:** `gossipsub` (Tracks `TEL:` hardware heartbeats and `SYNC_STATE:` Federation BFT replication).
* **Data Plane:** `libp2p::request_response` (1-to-1 Unicast TCP streams) extended with `FetchData` and `DataPayload`.
* **Storage Plane:** `libp2p::kad` (Kademlia DHT for VMFS file pinning).
* **Security:** Cryptographic `PeerId` generation via `/dev/urandom` mapped to `.swarm_identity`. TCP stream encryption via Libp2p `Noise`.
* **Consensus:** Dynamic Redundancy Factor (Max: 2) + SHA-256 Output State Hashing.

## 2. Distributed Stateful Smart Contracts
Stateful contracts execute sequentially and atomically, bypassing legacy MapReduce slicing.
1. **State Tracking:** The Gateway maintains a global `DashMap` of the latest verified `[contract_hash -> state_hash]`.
2. **Atomic Dispatch:** The Gateway packages the payload and dispatches it over WAN.
3. **Pre-Flight Sync:** The Worker verifies local `.state` hash against Gateway's expected hash, fetching via P2P if needed.
4. **Memory Injection:** `judge` instantiates `wasmi` and explicitly overwrites linear memory offset `0` with `.state` bytes.
