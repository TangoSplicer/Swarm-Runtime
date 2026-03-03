​Swarm Runtime: Alignment Protocol
​1. Core Mandates
​No Hallucinations: Verify all crates (axum 0.6, libp2p 0.51, wasmi 0.31) before suggesting code.
​Termux Native: All code must run on Android via Termux. No Docker.
​Atomic Progress: Features must be fully implemented and verified before moving to the next version.
​2. Project State (v0.21.1 Gold)
​Architecture: Hub-and-Spoke (Logical) / Mesh (Physical).
​Gateway: Handles Edge Cache mapping and Local Compiled Base64 payloads.
​Workers: Map VMFS Chroot jails, execute wasmi runtimes, Hash output files.
​CLI: Multi-route deployment (Edge cache vs Local Compile & Encode).
​3. Libp2p Network Buffer Axiom
​The Swarm strictly respects the libp2p::request_response CBOR codec stream limits (~2MB). Any compiled language generating heavily linked binaries (e.g., standard Go 1.26 at 2.5MB) is rejected.
​The Swarm enforces Zig or raw wabt compilation for native system logic to ensure sub-100KB bytecode that glides efficiently across the Unicast mesh.
