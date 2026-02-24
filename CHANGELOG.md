# Swarm Runtime Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.17.1] - 2026-02-24
### Added
- Deterministic Consensus algorithm with Byzantine Fault Tolerance.
- Redundancy Factor 2: Gateway assigns identical shards to multiple workers.
- Strict result matching and active ban-hammer for malicious nodes reporting fake results.
- Granular per-peer SLA tracking.

## [0.17.0] - 2026-02-24
### Added
- Wasmer `Singlepass` compiler integration to bypass Android/Termux AST recursion limits.
- `wasmer-middlewares` Gas Metering to prevent infinite loops (`loop {}`) and CPU exhaustion.

## [0.16.1] - 2026-02-23
### Added
- Lazy Assignment API architecture.
- Sysinfo hardware telemetry heartbeats over GossipSub.
- Weighted Sharding math allocation with remainder sweeping.

## [0.16.0] - 2026-02-23
### Added
- Ed25519 `SignedPayload` envelopes with 60-second TTL replay protection.
- 1-to-1 `libp2p::request_response` Unicast streams to replace noisy GossipSub data routing.
