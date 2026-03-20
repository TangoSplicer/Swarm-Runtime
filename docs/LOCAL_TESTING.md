# Swarm Runtime: Local & WAN Testing Guide (v1.0.0)

## 1. Native Chaos Testing (No Docker)
As of V1.0.0, we strictly reject Docker and external network proxies for simulating latency.
All integration testing is done natively in Rust using `libp2p::core::transport::memory::MemoryTransport`.

To run the automated BFT and cellular packet loss simulations:
```bash
cargo test --workspace --target x86_64-unknown-linux-gnu
2. Single-Device Integration Testing
​If testing end-to-end execution on a single physical device:
​Create separate directories: mkdir worker1_dir && worker2_dir.
​This isolates the .swarm_identity generation to prevent Libp2p PeerId collisions.
​Boot workers sequentially from their isolated directories and connect them to the local Gateway http://127.0.0.1:3000.
​3. Polyglot Runtime Verification
​Ensure the following files are present in the worker's execution directory to test WASI compliance: python.wasm, qjs.wasm, lua.wasm, ruby.wasm, php.wasm, sqlite.wasm.
