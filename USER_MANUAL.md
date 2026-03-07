# Swarm Runtime: Developer User Manual (v0.24.0)

## 1. Starting the Mesh & Cryptographic Identity
As of v0.24.0, every node dynamically generates a permanent cryptographic identity (`.swarm_identity`) on its first boot. 

To simulate a multi-device BFT mesh locally:
* **Gateway:** `cargo run --bin swarm-node -- gateway --port 3000`
* **Worker 1:** `cd worker1_dir && cargo run --manifest-path ../Cargo.toml --bin swarm-node -- start --shard 1`
* **Worker 2:** `cd worker2_dir && cargo run --manifest-path ../Cargo.toml --bin swarm-node -- start --shard 2`
*(Note: Because of the unique identity generation, Workers must be run in separate directories during local testing to avoid PeerId collisions).*

## 2. Deploying Stateful Smart Contracts
The CLI multiplexes deployments. Use `--lang` to route it correctly:
* **Python:** `cargo run --bin swarm-cli -- deploy app.py --lang python`
* **Zig (Auto-compiles):** `cargo run --bin swarm-cli -- deploy app.zig --lang zig`
* **Raw WebAssembly:** `cargo run --bin swarm-cli -- deploy native.wasm --lang wasm`
*(Note: Workers will automatically detect if their local state is out-of-sync and download the latest `.state` snapshot from peers before executing).*

## 3. Checking Status & Consensus
Query the Gateway to view distributed execution status and the resulting Output Hash.
`cargo run --bin swarm-cli -- status <JOB_ID>`

## 4. Fetching Data from the Mesh
Once a job achieves `Status: COMPLETED`, you can download the resulting output file directly from the decentralized Worker's Virtual File System using its Consensus Hash.
`cargo run --bin swarm-cli -- fetch <CONSENSUS_HASH>`
*(The file will be saved locally as `download_<hash>.bin`)*
