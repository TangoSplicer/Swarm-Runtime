# Swarm-Runtime: Technical Documentation

## The Decoupled Network Brain
1. **The Control Plane (GossipSub):** Lightweight mesh-wide state. Workers broadcast `TEL:` heartbeats.
2. **The Data Plane (Request-Response):** Direct point-to-point TCP streams for heavy Wasm payloads.
3. **The Storage Plane (Kademlia DHT):** Distributed file metadata pinning for the Virtual Mesh File System.

## Universal Payloads & WebAssembly Memory
To break free from primitive integer constraints, the Swarm `Judge` component acts as the Memory Manager for the WebAssembly guest.
1. The dataset `Vec<String>` is joined into a continuous UTF-8 byte array.
2. The Rust host calculates the required Wasm pages (64KB each) and allocates a `Memory` instance.
3. The bytes are injected into a Host-Managed Fixed Buffer starting at an offset of **1MB** to prevent overwriting the guest's internal stack/heap.
4. The host passes the `(pointer, length)` to the guest `execute()` function.

## Hash-Based Deterministic Consensus
Returning massive JSON strings or files over the network for consensus comparison would cause instant network congestion and OOM crashes.
Instead, we use **Output State Consensus**:
1. The Worker executes the payload.
2. The Worker hashes the output state and any modified files using **SHA-256**.
3. Only the 32-byte hash is returned to the Gateway.
4. The Gateway compares the hashes from multiple redundant shards to ensure execution honesty before accepting the state.

## The ARM64 Architecture Pivot (Wasmi)
To ensure maximum compatibility across mobile edge nodes (Android/aarch64), the runtime strictly utilizes the `wasmi` interpreter. Standard JIT compilers (like Wasmer/Singlepass) create memory alignment hardware traps during guest-to-host memory copying on strict ARM processors. The virtual stack approach of Wasmi provides 100% stable execution environments across heterogeneous mobile hardware.
