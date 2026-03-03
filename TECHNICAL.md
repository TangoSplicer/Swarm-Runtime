# Swarm-Runtime: Technical Documentation

## Defeating Network Bloat: The Zig Pivot
During Phase 5.6, natively compiling Go 1.26 to the WebAssembly System Interface (`wasip1`) generated highly bloated 2.5MB binaries that exceeded the internal limits of `libp2p::request_response` standard CBOR codecs. To avoid rewriting core networking abstractions, the Swarm utilizes **Zig**. Zig acts as an ultra-lean LLVM frontend that compiles systems-level logic into bare-metal ~5KB binaries, easily gliding through the mesh TCP streams.

## MapReduce and The Empty Shard Trap
The Swarm uses a Weighted Sharding algorithm designed for massively parallel datasets. The Scheduler calculates the number of tasks by dividing the `dataset` array length by the Redundancy Factor. 
When compiled Wasm modules were introduced, the CLI initially submitted an empty array (`vec![]`), causing the Scheduler to evaluate `0 / 2 = 0 tasks`. This resulted in silent network drops. The CLI now injects a `EXECUTE_NATIVE_WASM` dummy trigger to force the Scheduler to un-pause the Dispatcher thread.

## Hash-Based Deterministic Consensus
Returning massive JSON strings or files over the network for consensus comparison would cause instant network congestion and OOM crashes. Instead, we use **Output State Consensus**:
1. The Worker executes the polyglot/compiled payload.
2. The Worker hashes the output state and any modified files using **SHA-256**.
3. Only the 32-byte hash is returned to the Gateway to evaluate execution honesty.
