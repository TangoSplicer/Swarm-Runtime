# Swarm Runtime: Technical Specification v1.0.0

## 1. System Architecture
* **Topology:** Physical Mesh (Libp2p), Logical Star (Public Cloud Gateway-Coordinator).
* **Control Plane:** `gossipsub` (Tracks `TEL:` hardware heartbeats and `SYNC_STATE:` Federation BFT replication).
* **Data Plane:** `libp2p::request_response` (1-to-1 Unicast TCP streams) extended with `FetchData` and `DataPayload`.
* **Storage Plane:** `libp2p::kad` (Kademlia DHT for VMFS file pinning).
* **Fault Tolerance:** `Lazarus` async MPSC monitoring engine.

## 2. Distributed Stateful Smart Contracts & CRDTs
Stateful contracts execute concurrently across up to 50 nodes and merge deterministically.
1. **VMFS Sandboxing:** `judge` instantiates `wasmi` and mounts state securely via `cap-std` Virtual File System POSIX descriptors.
2. **Delta Extraction:** Workers parse the execution output into a `BTreeMap<String, String>` to inherently sort keys lexically.
3. **CRDT Merging:** Gateways merge colliding deltas mathematically (Commutative addition for numbers, Last-Write-Wins timestamps for strings).
4. **Deterministic Consensus:** The sorted merged BTreeMap is serialized and SHA-256 hashed for global Byzantine Fault Tolerance (BFT) verification.
