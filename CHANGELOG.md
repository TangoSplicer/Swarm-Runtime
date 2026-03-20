# Swarm Runtime Changelog


# Swarm Runtime Changelog

## [1.0.0] - 2026-03-20 (The Enterprise Release)
### Added
- **Multi-Shard State Merging (CRDT):** Gateways now mathematically merge execution outputs from up to 50 edge nodes using Commutative addition and Last-Write-Wins (LWW) conflict resolution.
- **Lazarus Fault Tolerance:** Integrated an asynchronous MPSC monitoring engine to auto-restart crashed sub-routines without blocking the Tokio event loop.
- **WASI Standardization:** Upgraded to strict `cap-std` VMFS sandboxing. State is now securely mounted via standard POSIX file descriptors, completely deprecating legacy memory-buffer injection.
- **Enterprise CI/CD:** GitHub Actions pipeline established utilizing `cross-rs` for automated `aarch64-linux-android` cross-compilation testing.
- **WORA Cryptography:** Replaced fragile Linux-only `/dev/urandom` reads with the cross-platform `rand::rngs::OsRng`.

### Changed
- **Network Resilience:** Removed excessive WAN mDNS chatter to save Android battery life.
- **Security Hardening:** Implemented strict `ed25519-dalek` signature verification on Workers before payload execution.
- **Algorithmic Scaling:** Transitioned tracking arrays to `HashSet`s for guaranteed O(1) shard lookups.
- **Deprecation:** Removed the `prism` AST compiler crate to fully align with the polyglot WASI execution model.

## [0.26.x +] 
### I honestly forgot to update the file between these. 


## [0.26.0] - 2026-03-15
### Added
- Enterprise Federation: Gateways now operate in Active-Active HA mode, replicating BFT consensus states via the `swarm-gateway-sync` Gossipsub topic.
- High Availability Routing: The Developer CLI and Android Workers now accept multiple, comma-separated Gateway URLs/Multiaddrs. If the primary Gateway fails, they automatically fall back to the secondary.
- Deterministic Sharding: Replaced the non-deterministic `DefaultHasher` in the Prism Compiler with mathematically sound `sha2` (SHA-256) to ensure cross-architecture BFT consensus.
### Changed
- Security Hardening: Locked Libp2p Gossipsub `ValidationMode` to `Strict` to prevent malicious actors from poisoning the control plane with unverified payloads.
- Memory Protection: Slashed the Libp2p `request_response` timeout from 300s to 15s to prevent massive asynchronous RAM leaks during Worker cellular disconnects.

## [0.25.5] - 2026-03-14
### Added
- Networking: Global Mesh Ignition! Successfully deployed the Gateway to an Oracle Cloud VPS (Public WAN).
