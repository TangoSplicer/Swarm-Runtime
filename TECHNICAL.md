# Swarm-Runtime: Technical Documentation & Post-Mortems

## The PKI Trap: Application vs Network Identity (Phase 9)

During the transition to public mesh readiness, the Swarm encountered an identity fracture. We successfully generated a permanent `ed25519` keypair for the Application Layer (signing job payloads), but the `SynapseNode` (Libp2p) was still generating a random Network Layer identity on every boot.
**The Trap:** If we gave the Worker its own unique identity, its local Application Layer `VerifyingKey` would reject the Gateway's payload signatures.
**Solution:** We injected the permanent 32-byte seed directly into the Libp2p `SwarmBuilder`. We then deprecated the manual payload signature verification, relying entirely on Libp2p's native `Noise` cryptographic protocol. The `Noise` handshake automatically encrypts and authenticates all TCP connections between known peers, securing the data plane without redundant application-layer checks.

## The BFT Deadlock: Deduplication vs Redundancy (Phase 8)
When implementing Job Deduplication (preventing a single worker from claiming multiple shards of the same job), a mathematical paradox occurred. The Gateway, running Redundancy Factor 2, split a job into Shard 0 and Shard 1. It expected 4 total executions (2 per shard). With only 2 workers online, Worker 1 took Shard 1, Worker 2 took Shard 0, and both locked the job in their `DashSet` cache. The Gateway hung indefinitely. 
**Solution:** Implemented **Atomic Routing**. Stateful contracts are no longer split. The entire payload is wrapped into a single Shard 0, allowing both workers to compute the exact same shard concurrently, fulfilling redundancy mathematically.

## The Late-Joiner State Divergence
In a stateful distributed system, if Worker 3 joins a network that has already incremented a contract 50 times, Worker 3's initial execution will start from `0`, resulting in a Hash Collision and a Gateway Ban.
**Solution:** Implemented P2P Pre-Flight State Synchronization. The Gateway appends `|STATE:<hash>` to payloads. Workers verify their local state hash matches before execution, downloading the correct state from peers via Libp2p Request-Response if empty.

## Intra-Worker Asynchronous Corruption
If a 64-core Worker eagerly claims multiple jobs targeting the *same* smart contract, the concurrent `tokio::spawn` threads will corrupt the `.state` file during simultaneous disk I/O.
**Solution:** Implemented a global Lock Manager using `tokio::sync::Mutex` mapped to Contract IDs. Concurrent threads on the same worker queue patiently to mutate state sequentially.
