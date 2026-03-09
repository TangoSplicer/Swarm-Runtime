# Swarm Runtime Changelog

## [0.25.0] - 2026-03-09
### Added
- Networking: Global Mesh Ignition! Successfully deployed the Gateway to an Oracle Cloud VPS (Public WAN).
- Networking: Hardcoded explicit Libp2p Multiaddr dialing in the Worker to bypass the local-only mDNS trap.
- API: Validated Axum REST API JSON deserialization and Gossipsub task routing across the public internet.

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
