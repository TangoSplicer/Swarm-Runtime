# Swarm Runtime: Developer User Manual (v0.20.9)

Welcome to the Swarm Polyglot Mesh. This guide outlines how to deploy interpreted language scripts across the decentralized network.

## 1. Starting the Mesh
Swarm requires at least one Gateway and two Workers to establish Redundancy Factor 2.
* **Gateway:** `cargo run --bin swarm-node -- gateway --port 3000`
* **Worker 1:** `cargo run --bin swarm-node -- start --shard 1`
* **Worker 2:** `cargo run --bin swarm-node -- start --shard 2`

## 2. Deploying Polyglot Code
Use the `swarm-cli` to deploy scripts. The CLI automatically attaches the correct `POLYGLOT` routing ID, and the workers will boot the corresponding WASI engine.

**Python**
`cargo run --bin swarm-cli -- deploy app.py --lang python`

**JavaScript / TypeScript (QuickJS)**
`cargo run --bin swarm-cli -- deploy app.js --lang js`

**Lua**
`cargo run --bin swarm-cli -- deploy app.lua --lang lua`

**Ruby**
`cargo run --bin swarm-cli -- deploy app.rb --lang ruby`

**PHP**
`cargo run --bin swarm-cli -- deploy app.php --lang php`

**SQLite (Serverless DB)**
`cargo run --bin swarm-cli -- deploy schema.sql --lang sqlite`

## 3. Checking Cryptographic Consensus
When deployment succeeds, you receive a Job ID. Query the Gateway to view the distributed execution status and output hashes.
`cargo run --bin swarm-cli -- status <JOB_ID>`

**Expected Output:**
```text
=== 📊 Swarm Job Status ===
Status:          COMPLETED
Consensus Hash:  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
Numeric Result:  0
Shard Breakdown: [(1, 0), (0, 0)]
Verified Shards: 2
===========================
