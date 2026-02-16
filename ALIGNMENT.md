# Swarm Runtime: Alignment Protocol

## 1. Core Mandates
* **No Hallucinations:** Verify all crates (axum 0.6, libp2p 0.51, wasmer 4.2) before suggesting code.
* **Termux Native:** All code must run on Android via Termux. No Docker.
* **Atomic Progress:** Features must be fully implemented and verified before moving to the next version.

## 2. Project State (v0.9.3 Gold)
* **Architecture:** Hub-and-Spoke (Logical) / Mesh (Physical).
* **Gateway:** `http://127.0.0.1:3000` (Axum). Handles Wasm uploads via "Atomic Broadcast".
* **Workers:** Libp2p nodes subscribing to `swarm-shard-1`.
* **Runtime:** Wasmer `Judge` executing `(i32, i32) -> i32`.
* **Telemetry:** Real-time Dashboard with `Shard ID -> Result` table.

## 3. The "Silent Error" Law
* **Never** use `.unwrap_or(-1)` silently. Always log the error trace.
* **Always** match Wasm signatures (`i32`) to the Rust runner.
