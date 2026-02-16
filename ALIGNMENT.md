# Swarm Runtime: Alignment & Operational Protocol

## 1. Core Mandates
* **No Hallucinations:** Code must be verified against the actual Rust/Axum/Libp2p crates used. Do not invent APIs.
* **Termux First:** All code must run natively on Android via Termux. No Docker, no heavy virtualization unless specified.
* **Atomic Progress:** Do not implement "half-features". Features must be fully implemented, compiled, and verified before moving to the next.

## 2. Project State (v0.9.3 Gold)
* **Current Version:** v0.9.3
* **Architecture:**
    * **Gateway (Orchestrator):** Axum-based HTTP server (Port 3000) that accepts Wasm uploads, shards them, and aggregates results.
    * **Worker (Shard):** Libp2p node that subscribes to `swarm-shard-1`, executes Wasm, and returns `SHARD_RESULT`.
    * **Runtime:** Wasmer-based `Judge` executing `(i32, i32) -> i32` functions.
* **Key Features:**
    * Atomic "Click-to-Broadcast" Wasm loading (bypasses mobile browser security).
    * Real-time Dashboard with Left-Side Heartbeats.
    * Live Result Aggregation Table.

## 3. The "Silent Error" Law
* **Never** use `.unwrap_or(-1)` without logging the error first.
* **Always** Verify Wasm signatures match the Runtime expectations (`i32` vs `u64` mismatch was the cause of the v0.9.2 incident).

## 4. Next Steps (Roadmap)
* **v0.10.0:** Fault Tolerance (Detect dropped nodes, re-assign shards).
* **v0.11.0:** Dynamic Registry (DHT-based peer discovery, removing the hardcoded loop).

