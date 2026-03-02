# Swarm-Runtime: Technical Documentation

## Polyglot WASI Routing & ABI Standardization
WebAssembly relies on the WebAssembly System Interface (WASI) to securely interact with the host OS (Android/Linux). During the Polyglot Phase, the Swarm unified all runtimes to the stable `wasi_snapshot_preview1` ABI. 
* **WASI Patching:** Legacy binaries utilizing the deprecated `wasi_unstable` ABI (e.g., Lua, SQLite) were natively decompiled via `wabt` to WebAssembly Text (`.wat`), patched with the new ABI string, and recompiled on the edge device to prevent Linker panics.
* **WASIp1 Alignment:** The Swarm officially supports the W3C's new `wasip1` nomenclature (utilized by the Ruby 3.2+ command module).

## Byzantine Fault Tolerance & The Hardened Judge
The Swarm incorporates a strict **"Silent Error Law"** within the `Judge` execution environment.
* `unwrap()` and `expect()` are strictly forbidden during module instantiation and memory allocation.
* If a WASI engine fails to boot, traps, or encounters a memory fault, the `Judge` safely catches the error via `Result<()>` mapping.
* The Worker gracefully degrades, calculates an `[ERROR]` hash state, and returns it to the Gateway. This allows the Gateway to achieve deterministic consensus on the failure state without the network crashing or hanging.

## Hash-Based Deterministic Consensus
Returning massive JSON strings or SQLite databases over the network for consensus comparison would cause network congestion.
1. The Worker executes the polyglot payload.
2. The Worker hashes the output state and newly written VMFS files using **SHA-256**.
3. Only the 32-byte hash is returned to the Gateway.
4. The Gateway compares hashes from multiple shards to establish execution honesty.
