# Swarm Runtime: Single-Device Testing Guide

When developing or testing the Swarm Runtime on a single physical device (e.g., opening multiple terminal tabs on one laptop or one Termux environment), you must account for the **Shared File System Race Condition**.

## The Shared Hard Drive Problem
By design, the Swarm maps a Virtual Mesh File System (VMFS) to the host device's local storage (typically `./rootfs/data`). In a production network, Worker 1 and Worker 2 are running on completely different physical devices, meaning their `rootfs/data` directories are completely isolated.

However, when you run multiple Workers on the *same* computer, they share the exact same physical folder. 

## The Byzantine Fault Tolerance (BFT) Trigger
If the Gateway dispatches a task with a **Redundancy Factor of 2** (the default), both local Workers will execute the code concurrently. 
1. Both WebAssembly sandboxes will attempt to write to `./rootfs/data/output.txt` at the exact same millisecond.
2. Worker 1 will overwrite the file while Worker 2 is trying to read it.
3. Because the read/write operations collide, Worker 1 and Worker 2 will generate different SHA-256 Hashes for the exact same task.
4. The Gateway receives mismatched hashes, assumes one of the nodes is corrupted or malicious, and permanently bans both nodes from the network to protect the data state (`🚨 HASH COLLISION DETECTED!`).

This is a feature, not a bug! It proves the Cryptographic Consensus model works perfectly.

## How to Test on a Single Device

To test successfully on a single machine, you must prevent the race condition using one of the following methods:

### Option A: Lower the Redundancy Factor (Recommended for Dev)
If you only care about testing execution and extraction, tell the Gateway to only generate 1 shard per task. This ensures only one Worker ever touches the file system at a time.
1. Open `components/swarm-node/src/gateway.rs`.
2. Locate the `JobState` initialization.
3. Change `redundancy: 2` to `redundancy: 1`.
4. Run only **one** Worker node alongside your Gateway.

### Option B: Isolated Directories (Simulating Production)
If you want to test true Redundancy Factor 2 consensus on a single machine, you must physically isolate the Workers so they don't share the same `rootfs`.
1. Compile the workspace.
2. Create two separate directories: `mkdir worker1_dir && mkdir worker2_dir`.
3. Copy the compiled `swarm-node` binary and WASI engines (`.wasm` files) into both directories.
4. Run Worker 1 from inside `worker1_dir`, and Worker 2 from inside `worker2_dir`. 
*(This allows both workers to create their own isolated `./rootfs/data` folders).*
