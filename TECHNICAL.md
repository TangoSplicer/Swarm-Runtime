# Swarm-Runtime: Technical Documentation & Post-Mortems

## Defeating the Tokio Async/Blocking Panic
During the Phase 6 CLI upgrade, integrating a synchronous `reqwest::blocking::Client` into an active `tokio` runtime resulted in immediate thread abortion. Tokio strictly forbids thread-blocking operations within its async thread pool to prevent deadlocks. The `swarm-cli` was entirely refactored to use `reqwest::Client` combined with `#[tokio::main]`, shifting the CLI into a purely asynchronous networking model.

## The Empty Hash Trap
When evaluating output consensus, the `judge` executes a strict SHA-256 hash against `./rootfs/data/output.txt`. If a deployed WebAssembly module writes to standard output (`stdout`) instead of the file system, the Judge calculates the hash of an empty buffer (or a fallback string). The resulting hash (`e3b0c442...`) represents zero data. All Wasm payloads must explicitly open and write to `output.txt` within their WASI sandbox to be retrievable by the DHT.

## The Shared-Drive Race Condition (BFT Security Event)
When testing a distributed network on a single physical device (e.g., Termux), running multiple Workers causes them to share the host's physical `./rootfs/data` directory. 
Because the Gateway's Redundancy Factor (RF=2) dispatches duplicate shards simultaneously, both `tokio::spawn` worker threads attempt to overwrite `output.txt` at the exact same millisecond. 
This read/write collision causes the Workers to return mismatched hashes to the Gateway. The Gateway's Byzantine Fault Tolerance algorithm detects this discrepancy, labels the nodes as compromised, and terminates their TCP connections. 
**Solution:** Single-device testing requires dynamically dropping the Redundancy Factor to 1, or isolating the Worker binary execution paths.
