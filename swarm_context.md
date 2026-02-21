
# Swarm Runtime: Project Context & Architecture

## 1. Project Overview
**Name:** Swarm Runtime
**Goal:** A mobile-first, fault-tolerant distributed edge-compute mesh built in Rust.
**Environment:** Actively developed and tested natively on an Android device using Termux. 
**Current Version:** `v0.13.1` (MapReduce & Memory Integration completed).

## 2. Core Tech Stack
* **Networking:** `rust-libp2p` (mDNS for discovery, GossipSub for messaging, Identify for TCP handshake stability, Kad for DHT routing).
* **API / Web:** `axum` and `tokio` (Headless Asynchronous REST API).
* **Execution:** `wasmer` (Sandboxed WebAssembly runtime with linear memory injection).
* **State Management:** `dashmap` for concurrent hash maps.

## 3. Directory Structure (Cargo Workspace)
```text
~/clones/swarm-runtime/
├── Cargo.toml                # Workspace definition
├── README.md, CHANGELOG.md, ROADMAP.md
├── components/
│   ├── swarm-node/           # Main binary application
│   │   ├── Cargo.toml
│   │   ├── src/main.rs       # CLI routing (Gateway vs Worker), Axum API, Libp2p event loop
│   │   └── src/sharding.rs   # Payload structs (ShardedDeployRequest, Shard)
│   ├── synapse/              # P2P Networking Layer
│   │   ├── Cargo.toml
│   │   └── src/lib.rs        # SynapseNode, SynapseBehavior (mDNS, Gossipsub, Identify, Kademlia)
│   ├── judge/                # WebAssembly Execution Engine
│   │   ├── Cargo.toml
│   │   └── src/lib.rs        # Wasm instantiation, Linear Memory injection, execution
│   └── prism/                # Shared utilities/data

## 4. Current Architecture (v0.13.1)
The system currently operates a successful MapReduce pipeline:
Submit: User sends a POST request to http://localhost:3000/api/v1/jobs containing a Wasm binary (Base64) and an integer array (dataset).
Gateway (Leader): Instantly returns a job_id. Slices the dataset into chunks based on active mDNS peer count. Publishes chunks to GossipSub topic swarm-shard-1.
Worker (Follower): Subscribes to swarm-shard-1. Validates if the shard_index matches its local Shard ID to prevent greedy execution.
Judge (Execution): The Worker passes the data chunk to Judge. Judge converts the Vec<i32> to bytes, writes it starting at offset 0 in the Wasm sandbox's linear memory, and calls the Wasm function with (pointer, length).
Reduce: Worker broadcasts the result back via GossipSub. The Gateway aggregates them in a DashMap.
Poll: User calls GET /api/v1/jobs/:id to retrieve the aggregated total_sum and breakdown.
## 5. Development Rules & Formatting
Strict Factuality: Always prioritize factual correctness over helpfulness. If Wasmer API or Libp2p API specifics are uncertain, check documentation or refuse to answer. Do not hallucinate code.
Termux Constraints: Complex bash variable expansions with Base64 strings can truncate in Termux. Rely on hardcoded, statically generated payloads for testing.
Code Delivery: Provide code in single, complete, copy-pasteable bash blocks utilizing cat > filepath << 'EOF' to overwrite files safely in Termux.
