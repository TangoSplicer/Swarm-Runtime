# Swarm Runtime: Single-Device Testing Guide

When testing the Swarm Runtime on a single physical device (e.g., multiple Termux tabs), you must account for the **Shared File System Race Condition**.

## The Byzantine Fault Tolerance (BFT) Trigger
If the Gateway dispatches a task with a Redundancy Factor of 2, two local Workers will execute the code concurrently. If they share the same `./rootfs/data` folder, they will corrupt the `.state` file during disk I/O, generate mismatched SHA-256 hashes, and trigger the Gateway to ban them.

## How to Test on a Single Device

### Option A: Isolated Directories (Simulating Production - RECOMMENDED)
Because Phase 8 relies on Atomic Routing for Smart Contracts (both workers execute the exact same Shard 0), you **must** physically isolate the Workers so they don't share the same `rootfs`.
1. Compile the workspace.
2. Create two separate directories: `mkdir worker1_dir && mkdir worker2_dir`.
3. Copy the compiled WASI engines (`.wasm` files) into both directories.
4. Run Worker 1: `cd worker1_dir && cargo run --manifest-path ../Cargo.toml --bin swarm-node -- start --shard 1`
5. Run Worker 2: `cd worker2_dir && cargo run --manifest-path ../Cargo.toml --bin swarm-node -- start --shard 2`
*(This allows both workers to manage their own `.state` files, properly testing P2P state synchronization).*

### Option B: Lower the Redundancy Factor
If you only care about testing execution and extraction, open `gateway.rs` and change `redundancy: 2` to `redundancy: 1`. 
*(Note: This bypasses BFT consensus and should only be used for rapid syntax debugging).*
