​Swarm Runtime: Alignment Protocol
​1. Core Mandates
​No Hallucinations: Verify all crates (axum 0.6, libp2p 0.51, wasmi 0.31) before suggesting code.
​Termux Native: All code must run on Android via Termux. No Docker.
​Atomic Progress: Features must be fully implemented and verified before moving to the next version.
​2. Project State (v0.20.9 Gold)
​Architecture: Hub-and-Spoke (Logical) / Mesh (Physical).
​Gateway: Handles Polyglot Wasm uploads and Dynamic Redundancy Factor fallbacks.
​Workers: Libp2p nodes that dynamically load WASI interpreters (Python, JS, Lua, Ruby, PHP, SQLite) from local edge caches.
​Runtime: Wasmi Judge executing in strict ./rootfs chroot jails.
​Telemetry: CLI-driven job deployment and SHA-256 Hash-Based consensus checking.
​3. The "Silent Error" Law (ENFORCED)
​Never use .unwrap() silently.
​During v0.20.8, this law saved the network. All WASI booting sequences now use .map_err() to return graceful [ERROR] states, preventing Host thread panics from dropping peers.
​4. WASI ABI Alignment
​The Swarm strictly enforces the wasi_snapshot_preview1 (or modern wasip1) standard. Any binary using wasi_unstable must be patched via wabt before injection.
