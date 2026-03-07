# Swarm Runtime: Single-Device Testing Guide

When testing the Swarm Runtime on a single physical device (e.g., multiple Termux tabs), you must account for the **Shared File System Race Condition** and the **Identity Collision Protocol**.

## The Byzantine Fault Tolerance (BFT) Trigger
If the Gateway dispatches a task with a Redundancy Factor of 2, two local Workers will execute the code concurrently. If they share the same `./rootfs/data` folder, they will corrupt the `.state` file during disk I/O, generate mismatched SHA-256 hashes, and trigger the Gateway to ban them.

## The Identity Collision Protocol
As of v0.24.0, nodes dynamically generate a `.swarm_identity` file. If two workers run in the same directory, they will share the exact same cryptographic `PeerId`, causing the Libp2p network to collapse.

## How to Test on a Single Device

### Isolated Directories (Simulating Production - REQUIRED)
You **must** physically isolate the Workers so they don't share the same `rootfs` or `.swarm_identity`.
1. Compile the workspace.
2. Create two separate directories: `mkdir worker1_dir && mkdir worker2_dir`.
3. Copy the compiled WASI engines (`.wasm` files) into both directories.
4. Run Worker 1: `cd worker1_dir && cargo run --manifest-path ../Cargo.toml --bin swarm-node -- start --shard 1`
5. Run Worker 2: `cd worker2_dir && cargo run --manifest-path ../Cargo.toml --bin swarm-node -- start --shard 2`
*(This allows both workers to manage their own `.state` files and unique `.swarm_identity` keys, properly testing P2P state synchronization and network discovery).*
