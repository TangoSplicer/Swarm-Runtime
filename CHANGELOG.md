# Swarm Runtime Changelog

## [0.22.0] - 2026-03-04
### Added
- Core: Distributed Data Retrieval. CLI `fetch` command queries Gateway for specific SHA-256 hashes.
- Core: `synapse` Data Plane upgraded with `FetchData` and `DataPayload` CBOR routing.
- Worker: Decentralized file server capabilities. Workers actively sweep `./rootfs/data` to stream files over TCP Libp2p streams.
### Changed
- CLI: Converted from synchronous blocking architecture to a fully `tokio` asynchronous runtime to handle network data streaming.
- BFT: Verified Race Condition isolation behavior during single-device local testing.

## [0.21.1] - 2026-03-03
### Added
- Polyglot: Zig support via auto-local compilation (`wasm32-wasi -O ReleaseSmall`).
