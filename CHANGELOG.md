# Swarm Runtime Changelog

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
