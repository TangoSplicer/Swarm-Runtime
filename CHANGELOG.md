# Swarm Runtime Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.20.0] - 2026-02-27
### Added
- Virtual Mesh File System (VMFS): WASI integration to grant Wasm secure access to virtual sandboxed directories using `cap-std` ambient authority.
- VMFS Sweeper logic: Workers detect newly generated files post-execution and hash them.
- Kademlia DHT Integration: Workers successfully announce hashed file states to the decentralized mesh.
### Changed
- Replaced Wasmer JIT with `wasmi` interpreter to permanently bypass ARM64/aarch64 strict hardware memory alignment panics on Android.

## [0.19.0] - 2026-02-26
### Added
- Groundwork for WASI integration to support Virtual File Systems.
- Pre-flight scaffolding for Swarm decentralized storage nodes.

## [0.18.1] - 2026-02-26
### Changed
- Refactored `swarm-node` monolith into modular components: `main.rs`, `gateway.rs`, `worker.rs`, and `types.rs`.
- Cleaned up API routing and isolated `libp2p` networking boundaries.

## [0.18.0] - 2026-02-26
### Added
- Universal Payloads: Upgraded dataset ingestion to accept `Vec<String>` and complex JSON arrays.
- WebAssembly Linear Memory Injection: Host allocates memory and injects UTF-8 byte arrays directly into the Wasm guest at a safe 1MB offset.
- Hash-Based Consensus: Workers calculate a SHA-256 hash of their output execution state. Gateway verifies the hashes to establish deterministic consensus before accepting results.

## [0.17.1] - 2026-02-24
### Added
- Deterministic Consensus algorithm with Byzantine Fault Tolerance (Redundancy Factor 2).
- Granular per-peer SLA tracking.

## [0.17.0] - 2026-02-24
### Added
- Wasmer `Singlepass` compiler integration to bypass Android/Termux AST recursion limits.
- `wasmer-middlewares` Gas Metering to prevent infinite loops (`loop {}`) and CPU exhaustion.
