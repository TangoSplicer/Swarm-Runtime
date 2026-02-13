# Swarm-Runtime: Technical Documentation

## The Sandbox Bridge (Judge)
The `Judge` component acts as the execution supervisor. It uses the `Wasmer` runtime but restricts the module's capabilities. 

### Host Functions (The ABI)
Wasm modules are memory-isolated. To allow data persistence, we export the following symbols from the Rust host to the Wasm environment:
- `fn db_set(key: i32, val: i32)`: Writes a 4-byte integer to the local `sled` partition.
- `fn db_get(key: i32) -> i32`: Retrieves a 4-byte integer from the local partition.

## The Network Brain (Synapse)
The networking layer uses a "Gossip-then-Execute" pattern.
1. **Gateway** receives a Wasm payload via the Axum HTTP interface.
2. It publishes a `TASK:[ID]:[BASE64]` message to the GossipSub topic `swarm-shard-X`.
3. **Workers** subscribed to that shard download, decode, and execute the task.
4. **Results** are published back to the mesh and intercepted by the Gateway via a `oneshot` channel to close the HTTP request loop.

## Platform Compatibility
The core dependencies are purely Rust-based:
- **Networking:** `libp2p` (Standard cross-platform).
- **Storage:** `sled` (Pure Rust).
- **Runtime:** `Wasmer` (Native C/Rust).
This architecture is designed to compile on any OS supported by Rust.
