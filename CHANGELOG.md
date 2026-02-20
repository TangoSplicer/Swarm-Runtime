# Swarm Runtime Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.13.1] - 2026-02-20
### Added
- Wasm linear memory byte injection inside `Judge` to pass dynamic datasets directly into the sandbox.
### Fixed
- "Greedy Worker" routing bug: Workers now strictly compute only their assigned `shard_index`.
- Unused variable compiler warnings across the codebase.

## [0.13.0] - 2026-02-20
### Changed
- `ShardedDeployRequest` JSON payload upgraded to support dynamic integer arrays (`dataset`).
- Gateway data routing engine mathematically slices arrays based on the active peer count.

## [0.12.0] - 2026-02-20
### Added
- Asynchronous API endpoints (`POST /api/v1/jobs` and `GET /api/v1/jobs/:id`).
- Real-time terminal logging for WebAssembly runtime traps to improve debugging visibility.
### Changed
- Refactored `Judge` to safely auto-detect Wasm exports regardless of the internal function name.
### Removed
- Synchronous blocking deployment architecture in the Axum web server.

## [0.11.0] - 2026-02-20
### Added
- Libp2p `Identify` protocol to stabilize TCP handshakes and prevent silent protocol drops.
- 5-second `pending_dials` DashMap state to prevent mDNS connection flooding.
### Fixed
- TCP simultaneous open race conditions resolved via a deterministic Peer ID alphabetical Tie-Breaker.
