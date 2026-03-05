# Swarm Runtime: Developer User Manual (v0.22.0)

## 1. Starting the Mesh
* **Gateway:** `cargo run --bin swarm-node -- gateway --port 3000`
* **Worker 1:** `cargo run --bin swarm-node -- start --shard 1`
* **Worker 2:** `cargo run --bin swarm-node -- start --shard 2`

## 2. Deploying Code
The CLI multiplexes deployments. Use `--lang` to route it correctly:
* **Python:** `cargo run --bin swarm-cli -- deploy app.py --lang python`
* **Zig (Auto-compiles):** `cargo run --bin swarm-cli -- deploy app.zig --lang zig`
* **Raw WebAssembly:** `cargo run --bin swarm-cli -- deploy native.wasm --lang wasm`

## 3. Checking Status & Consensus
Query the Gateway to view distributed execution status and the resulting Output Hash.
`cargo run --bin swarm-cli -- status <JOB_ID>`

## 4. Fetching Data from the Mesh
Once a job achieves `Status: COMPLETED`, you can download the resulting output file directly from the decentralized Worker's Virtual File System using its Consensus Hash.
`cargo run --bin swarm-cli -- fetch <CONSENSUS_HASH>`
*(The file will be saved locally as `download_<hash>.bin`)*
