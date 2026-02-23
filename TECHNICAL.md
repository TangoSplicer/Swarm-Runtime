# Swarm-Runtime: Technical Documentation

## The Decoupled Network Brain (Synapse)
The networking layer has evolved from a noisy broadcast mesh into a highly efficient dual-plane architecture:

1. **The Control Plane (GossipSub):** Lightweight mesh-wide state. Workers broadcast `TEL:` heartbeats containing system load.
2. **The Data Plane (Request-Response):** Direct point-to-point TCP streams. Transmits megabytes of Wasm data to specific nodes without saturating the mobile network. Employs a "Decoupled ACK" strategy to prevent TCP Half-Open timeouts during long Wasm executions.

## The Mobile-Aware Scheduler
To prevent dying phones from bottlenecking the distributed mesh, the Gateway employs Lazy Assignment and Weighted Sharding. 

### The Allocation Math
When telemetry arrives, the Gateway calculates a Fitness Score for each active node:
`Fitness = free_ram_mb / (cpu_load + 1.0)`

The dataset is divided proportionally based on these weights using `floor()` math. To prevent data loss from floating-point truncation, the scheduler enforces a `minimum_chunk_size` of `1` and applies a remainder sweep to the final node in the active peer list.

## Cryptographic Security Layer
All Unicast payloads are wrapped in a `SignedPayload` envelope.
- **Root of Trust:** Gateway holds an `Ed25519` private key. Workers hold the public key.
- **Replay Protection:** Payloads include an `expires_at` Unix timestamp (60-second TTL).
- **Ban Hammer:** If a worker receives an invalid signature, malformed envelope, or expired TTL, it actively invokes `disconnect_peer_id` to ban the malicious actor from the mesh.
