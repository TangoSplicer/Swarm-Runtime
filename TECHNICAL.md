# Swarm-Runtime: Technical Documentation

## The Decoupled Network Brain (Synapse)
1. **The Control Plane (GossipSub):** Lightweight mesh-wide state. Workers broadcast `TEL:` heartbeats.
2. **The Data Plane (Request-Response):** Direct point-to-point TCP streams for heavy Wasm payloads, preventing broadcast storms.

## Execution Security & The Sandbox
Executing untrusted code on edge devices requires extreme security.
* **Singlepass Compiler:** We utilize Wasmer's linear-time `Singlepass` compiler. It protects against JIT-bombs and drastically reduces memory overhead, preventing Android's Low Memory Killer (LMK) from terminating the node.
* **Gas Metering:** The `judge` component attaches a middleware that assigns a cost to every Wasm instruction. If a module hits the 5,000,000 limit (e.g., an infinite `loop {}`), it instantly traps and terminates, saving mobile battery.

## Deterministic Consensus (Anti-Cheat)
To solve the Byzantine General's Problem, the Gateway implements Redundant Verification:
1. `Redundancy Factor = 2`.
2. Identical Wasm payloads are dispatched to multiple independent peers.
3. The Gateway buffers `raw_results`.
4. If unique results `== 1`, the result is migrated to `verified_results`.
5. If results conflict, a `CONSENSUS FAILURE` is triggered, the data is discarded, and all conflicting peers are disconnected from the mesh.
