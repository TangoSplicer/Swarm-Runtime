# Swarm Runtime: Roadmap to V1.0 🚀

## Phase 1 to 9.1: Core Engine & PKI Identity - ✅ COMPLETED
- Fault Tolerance, Async Axum API, Hash-Based Consensus.
- P2P Pre-Flight State Sync & Atomic Smart Contract Routing.
- Cryptographic Identity (`.swarm_identity`) and unified Network IDs via Libp2p `Noise`.

## Phase 9.2: The Global Mesh (Oracle Cloud) - ✅ COMPLETED
- **Public Cloud Gateway:** Deployed the `swarm-node` Gateway to a public Oracle Cloud VPS.
- **NAT Traversal:** Connected Termux mobile edge workers to the public Gateway over the WAN, proving real-world distributed BFT consensus.

## Phase 10: Swarm Hardening (Reliability) - ✅ COMPLETED
- **Memory Limits & Pruning:** Eradicated Base64 memory overhead; implemented raw `Vec<u8>` state hashing.
- **Asynchronous Backpressure:** Transitioned to bounded `tokio::sync::mpsc::channel(1000)` queues with `try_send` load-shedding to gracefully reject traffic floods.
- **Network Resilience:** Implemented strict network timeouts across CLI and Gateway boundaries to handle cellular packet loss.
- **ARG_MAX Bypass:** Transitioned to `reqwest::multipart` to stream raw binaries over Axum without crashing Android terminals.

## Phase 11: Polyglot ## Phase 11: Polyglot & Integration Verification - 🚧 NEXT Integration Verification - ✅ COMPLETED
- **Language Sandbox Testing:** Verified execution, state retention, and DHT pinning for all supported languages: Python, JavaScript, Lua, Ruby, PHP, SQLite, Go, and Zig.
- **Failure Condition Audits:** Simulated edge-case network drops during heavy multi-shard Polyglot executions.

## Phase 12: Enterprise Federation (Production Readiness) - ✅ COMPLETED
- **Gateway Replication:** Eliminated the single point of failure by implementing Gossipsub Active-Active `DashMap` state replication.
- **HA Routing:** Mobile workers auto-dial secondary gateways upon connection drops.

## Phase 13: Core Hardening & Tech Debt Resolution - 🚧 NEXT
- **Payload & Path Security:** Enforce cryptographic signature verification on Edge Workers and implement Cap-Std VMFS path sanitization to prevent directory traversal.
- **Tokio Deadlock Eradication:** Refactor the Gateway DashMap scheduling loops to strictly enforce the clone-and-release pattern.
- **WASI Standardization:** Eliminate byte-concatenation state injection in favor of secure, sandboxed POSIX file mounts.
- **Algorithm & Network Optimization:** Eradicate O(n²) shard detection bottlenecks and implement adaptive Gossipsub telemetry backoff to prevent network flooding.
- **API & UX Polish:** Connect the Axum router to the frontend dashboard, enforce strict HTTP payload validation, and dynamically sync version strings.

## Phase 14: Multi-Shard State Merging - 🚧 NEXT
- **Cryptographic Merging:** Build the logic to take multiple distributed execution shards and cryptographically merge their output states back together into a single master database.

## Phase 15: Telemetry Control Center (V1.0 Prep) - ⏳ PLANNED
- **Edge Dashboard:** Build a lightweight HTML/JS dashboard served natively by the Axum Gateway to visually track active Termux workers, hardware metrics, and federated job routing in real-time.

## Phase 16: Security & Sandboxing Hardening - ⏳ PLANNED
- **CPU Instruction Metering:** Inject strict "gas limits" into the WebAssembly execution to prevent malicious infinite loop traps on Android nodes.
- **VMFS Quotas:** Strictly limit the Virtual File System (VMFS) capabilities to prevent storage exhaustion attacks.

## Phase 17: V1.0 Release Candidate - ⏳ PLANNED
- **Documentation & Installers:** Finalize developer documentation and one-click install scripts.
- **Global Stress Test:** Execute massive multi-node stress tests to guarantee BFT consensus holds under severe cellular packet loss.

---

# 🚀 Beyond V1.0: Universal Compilation (WORA)

## Phase 18: WebAssembly Component Model (WASI 0.2) - 🔮 FUTURE
- **Polyglot Linking:** Upgrade from single-file execution to the WASI Component Model. Allow developers to compile complex projects with database logic in Rust, networking in Go, and UI in JS into a single, unified `.wasm` LEGO block.
- **Interface Types (WIT):** Implement Wasm Interface Types to allow complex data structures (strings, nested arrays) to seamlessly cross the sandbox boundary natively.

## Phase 19: The Universal "Swarm App Wrapper" - 🔮 FUTURE
- **Embedded Engine:** Strip the CLI from `swarm-node` and compile the Libp2p/WASI engine as a background C-compatible library (`libswarm`).
- **Native GUI Integration:** Bundle `libswarm` with native OS rendering engines (via frameworks like Tauri or Egui). 
- **The WORA Deployment:** Allow developers to ship a single executable (`.exe`, `.app`, or `.apk`). When a user double-clicks the app, it renders a native UI while seamlessly joining the global Swarm compute mesh in the background.

## Phase 20: Desktop UI & GPU Acceleration - 🔮 FUTURE
- **WebGPU Delegation:** Expose GPU hardware acceleration through the WASI sandbox so universal applications can offload rendering or ML inference tasks to the local graphics card before routing heavy dataset merging to the external Swarm.
