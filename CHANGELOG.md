# Swarm Runtime Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

# Swarm Runtime Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

[0.21.1] - 2026-03-03
Added
Polyglot: Zig support via auto-local compilation (wasm32-wasi -O ReleaseSmall).
CLI dynamic toolchain interception for compiled payloads.

[0.21.0] - 2026-03-03
Added
Core: Direct base64 raw .wasm payload deployment via the REST API.
Fixed
"Empty Shard Trap": CLI now injects "EXECUTE_NATIVE_WASM" dataset triggers to force the MapReduce Scheduler to dispatch compiled binaries across the Libp2p network.

[0.20.9] - 2026-03-02
Added
Polyglot: SQLite serverless database integration via sqlite.wasm.
Changed
Decompiled and patched SQLite WASI ABI to strictly adhere to wasi_snapshot_preview1.

[0.20.8] - 2026-03-02
Security
Hardened the Judge component against Rust thread panics. Eliminated all .unwrap() calls during Wasm instantiation, enabling the Gateway to achieve consensus on failed states without network degradation.

[0.20.7] - 2026-03-02
Added
Polyglot: PHP 8.2 execution support via Wasm Labs slim binary.

[0.20.6] - 2026-03-02
Added
Polyglot: Ruby execution support utilizing the official wasip1 CLI Command Module.

[0.20.5] - 2026-03-02
Added
Polyglot: Lua execution support. Manually patched legacy wasi_unstable imports via wabt tooling on Android.

[0.20.4] - 2026-03-02
Added
Polyglot: JavaScript/TypeScript support via the QuickJS WebAssembly engine

## [0.20.3] - 2026-03-02
### Added
- `swarm-cli` developer tool for unified REST API deployments (`deploy`) and verification (`status`).
- Polyglot Edge Caching: Bypassed the 2MB libp2p payload limit using `POLYGLOT:LANG` identifiers to load massive interpreters directly from worker storage.
- WASI Chroot Jails: Refactored the `Judge` to map an isolated `./rootfs` environment, permanently resolving `Errno 76` capability traps caused by Android Termux symlinks.
- Increased Wasmi execution fuel limits to 50 Billion operations to support full CPython boots.

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
