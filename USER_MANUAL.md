# Swarm Runtime: Developer User Manual (v0.21.1)

Welcome to the Swarm Polyglot Mesh. This guide outlines how to deploy interpreted scripts and compiled native binaries across the decentralized network.

## 1. Starting the Mesh
Swarm requires at least one Gateway and two Workers to establish Redundancy Factor 2.
* **Gateway:** `cargo run --bin swarm-node -- gateway --port 3000`
* **Worker 1:** `cargo run --bin swarm-node -- start --shard 1`
* **Worker 2:** `cargo run --bin swarm-node -- start --shard 2`

## 2. Deploying Interpreted Code
The CLI automatically attaches the correct Edge Cache routing ID, and the workers will boot the corresponding WASI engine.

* **Python:** `cargo run --bin swarm-cli -- deploy app.py --lang python`
* **JavaScript:** `cargo run --bin swarm-cli -- deploy app.js --lang js`
* **Lua:** `cargo run --bin swarm-cli -- deploy app.lua --lang lua`
* **Ruby:** `cargo run --bin swarm-cli -- deploy app.rb --lang ruby`
* **PHP:** `cargo run --bin swarm-cli -- deploy app.php --lang php`
* **SQLite:** `cargo run --bin swarm-cli -- deploy schema.sql --lang sqlite`

## 3. Deploying Compiled Languages
For maximum performance, you can deploy bare-metal compiled logic. The CLI will locally compile the source code, encode it in Base64, and dispatch the raw bytecode to the mesh.

### Zig (Native Compilation)
Zig generates ultra-lean WebAssembly that respects network streaming limits. Ensure `zig` is installed locally.
`cargo run --bin swarm-cli -- deploy app.zig --lang zig`

### Raw WebAssembly (.wasm)
If you compiled your payload with another toolchain (e.g., `wat2wasm`, or `clang`), you can dispatch the `.wasm` file directly.
`cargo run --bin swarm-cli -- deploy native.wasm --lang wasm`

## 4. Checking Cryptographic Consensus
Query the Gateway using the ID provided during deployment to view the distributed execution status and output hashes.
`cargo run --bin swarm-cli -- status <JOB_ID>`
