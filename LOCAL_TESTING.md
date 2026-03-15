# Swarm Runtime: Local & WAN Testing Guide

## The Byzantine Fault Tolerance (BFT) Trigger
If the Gateway dispatches a task with a Redundancy Factor of 2, two Workers will execute the code concurrently. If they share the same `./rootfs/data` folder on a single device, they will corrupt the `.state` file.

## The Identity Collision Protocol
Nodes dynamically generate a `.swarm_identity` file. If two workers run in the same directory, they share the exact same cryptographic `PeerId`, causing a Libp2p collapse.

## How to Test on a Single Device
1. Create two separate directories: `mkdir worker1_dir && mkdir worker2_dir`.
2. Copy compiled WASI engines (`.wasm`) into both directories.
3. Boot workers sequentially from their isolated directories.

## Connecting to the Public WAN
When testing over cellular networks against the Cloud Gateway, ensure `worker.rs` explicitly dials the Oracle IP, as mDNS peer discovery will fail across the public internet.

## Polyglot Runtime Verification (Phase 11)
Before transitioning to Enterprise Federation, all WASI execution engines must be verified. 
Ensure the following files are present in the worker's execution directory and test them sequentially via the `swarm deploy` CLI:
* `python.wasm`
* `qjs.wasm`
* `lua.wasm`
* `ruby.wasm`
* `php.wasm`
* `sqlite.wasm`
