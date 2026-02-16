# Swarm Runtime: Technical Specification v0.9.3

## 1. Architecture: "The Starburst"
* **Pattern:** Hub-and-Spoke (Logical) / Mesh (Physical).
* **Data Flow:**
    1.  **User:** Uploads `.wasm` (via `multipart/form-data` or JSON-base64) to Gateway.
    2.  **Gateway:**
        * Calculates `PeerCount`.
        * Splits `JobRange` (e.g., 0-100) into chunks.
        * Broadcasts `SHARD: { task_id, wasm_bytes, start, end }` via GossipSub.
    3.  **Workers:**
        * Receive `SHARD` message.
        * Compile Wasm via `Wasmer`.
        * Execute `execute(start, end)`.
        * Publish `SHARD_RESULT: { task_id, result }`.
    4.  **Gateway:** Aggregates results into a `DashMap` and returns JSON to UI.

## 2. The Protocol (GossipSub)
* **Topic:** `swarm-shard-1`
* **Message Format:** String-based (for easy debugging).
    * `SHARD:<JSON_PAYLOAD>`
    * `SHARD_RESULT:<UUID>:<SHARD_IDX>:<VALUE>`

## 3. The Runtime (Judge)
* **Engine:** Wasmer
* **Signature:** Strict `(i32, i32) -> i32`
* **Memory:** Ephemeral (New instance per shard execution).

## 4. The Dashboard (UI)
* **Tech:** Server-Side Rendered HTML (via Axum) + Vanilla JS.
* **Key Mechanic:** "Atomic Broadcast"
    * File is read into memory *only* when "Broadcast" is clicked.
    * Prevents mobile browser security blocks on file inputs.
