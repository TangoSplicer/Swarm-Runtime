# Swarm-Runtime: Technical Documentation & Post-Mortems

## The BFT Deadlock: Deduplication vs Redundancy (Phase 8)
When implementing Job Deduplication (preventing a single worker from claiming multiple shards of the same job), a mathematical paradox occurred. The Gateway, running Redundancy Factor 2, split a job into Shard 0 and Shard 1. It expected 4 total executions (2 per shard). With only 2 workers online, Worker 1 took Shard 1, Worker 2 took Shard 0, and both locked the job in their `DashSet` cache. The Gateway hung indefinitely waiting for redundant checks that could never happen. 
**Solution:** Implemented **Atomic Routing**. Stateful contracts are no longer split. The entire payload is wrapped into a single Shard 0, allowing both workers to compute the exact same shard concurrently, fulfilling redundancy mathematically.

## The Late-Joiner State Divergence
In a stateful distributed system, if Worker 3 joins a network that has already incremented a contract 50 times, Worker 3's initial execution will start from `0`, resulting in a Hash Collision and a Gateway Ban.
**Solution:** Implemented P2P Pre-Flight State Synchronization. The Gateway appends `|STATE:<hash>` to smart contract payloads. Workers must verify their local state hash matches before execution, downloading the correct state from peers via Libp2p Request-Response if empty.

## The Shared-Drive Race Condition (BFT Security Event)
When testing a distributed network on a single physical device (e.g., Termux), running multiple Workers causes them to share the host's physical `./rootfs/data` directory, causing read/write collisions.
**Solution:** Single-device testing requires isolating the Worker binary execution paths into separate directories (`worker1_dir`, `worker2_dir`).

## Intra-Worker Asynchronous Corruption
If a 64-core Worker eagerly claims multiple jobs targeting the *same* smart contract, the concurrent `tokio::spawn` threads will corrupt the `.state` file during simultaneous disk I/O.
**Solution:** Implemented a global Lock Manager using `tokio::sync::Mutex` mapped to Contract IDs. Concurrent threads on the same worker queue patiently to mutate state sequentially.
