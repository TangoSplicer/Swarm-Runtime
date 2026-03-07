# Swarm Runtime Changelog

## [0.24.0] - 2026-03-07
### Added
- Security: Removed hardcoded `GATEWAY_SECRET_SEED`. Swarm nodes now dynamically generate permanent cryptographic identities via OS-native `/dev/urandom` and save them to `.swarm_identity`.
- Networking: Injected the permanent identity seed directly into the Libp2p `SynapseNode`, unifying the Application and Network layer identities for stable public IP addressing.
### Changed
- Security: Deprecated manual `ed25519_dalek` signature verification on incoming payloads in favor of Libp2p's native `Noise` cryptographic protocol for encrypted, authenticated TCP streams.

## [0.23.0] - 2026-03-06
### Added
- Core: P2P Pre-Flight State Synchronization. Late-joining workers query the Gateway's expected state hash and dynamically download missing 1MB memory snapshots from peers.
- Gateway: Atomic Stateful Routing. Smart contracts bypass MapReduce logic to ensure perfectly deterministic redundant execution.
- Gateway: Dynamic Redundancy Scaling. Gateway lowers redundancy automatically if peer count drops below SLA requirements.

## [0.22.1] - 2026-03-05
### Added
- Judge: Persistent Stateful Actors. Extracts and injects WebAssembly linear memory.
- Worker: `tokio::sync::Mutex` Contract Lock Manager prevents async thread corruption.
- Worker: Job Deduplication. Highly concurrent `DashSet` caching prevents workers from double-claiming redundant job shards.
### Changed
- Judge: Removed static 1MB payload pointers in favor of dynamic `total_memory - payload_length` algorithms to prevent state overwrites.

## [0.22.0] - 2026-03-04
### Added
- Core: Distributed Data Retrieval via asynchronous Libp2p streams.
