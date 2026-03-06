# Acknowledgements & Open Source Tribute 🏆

The Swarm Runtime is a testament to the power of open-source software. This project would be fundamentally impossible without the thousands of hours of engineering freely gifted to the world by the following communities, projects, and maintainers. 

We extend our deepest gratitude to:

## The Core Foundations
* **[Rust](https://www.rust-lang.org/):** For providing the memory safety, fearless concurrency, and zero-cost abstractions that allow this decentralized mesh to run reliably on resource-constrained mobile devices.
* **[Termux](https://termux.dev/):** For turning Android into a first-class Linux development environment and proving that mobile phones are incredibly capable edge servers.

## The Decentralized Mesh
* **[rust-libp2p](https://libp2p.io/):** The absolute backbone of the Swarm. Thank you for the Kademlia DHT, Gossipsub, Request-Response protocols, and the modular networking stack that enables our devices to dynamically discover each other and reach consensus without a central database.

## The Asynchronous API & Concurrency
* **[Tokio](https://tokio.rs/):** The asynchronous runtime that breathes life into our network, handling massive concurrency, network streams, and thread-safe MPSC channels.
* **[Axum](https://github.com/tokio-rs/axum):** For the beautifully ergonomic, headless REST API that bridges the standard web to our P2P mesh.
* **[DashMap](https://github.com/xacrimon/dashmap):** For the incredibly fast, highly concurrent data structures that power our Stateful Contract Lock Managers and Job Deduplication caches.

## The WebAssembly Execution Engine
* **[Wasmi](https://github.com/wasmi-labs/wasmi):** The brilliant, pure-Rust WebAssembly interpreter. Your lightweight design is the exact reason our nodes can execute deterministic smart contracts safely on ARM64 mobile processors.
* **[Cap-std](https://github.com/bytecodealliance/cap-std):** For the capability-based security model that allows us to strictly sandbox the Virtual Mesh File System (VMFS) and protect host devices from malicious code.

## Cryptography & Telemetry
* **[ed25519-dalek](https://github.com/dalek-cryptography/ed25519-dalek):** For the blazing-fast digital signatures that secure our payload routing and prevent replay attacks.
* **[RustCrypto (sha2)](https://github.com/RustCrypto/hashes):** For the SHA-256 hashing algorithms that make our Byzantine Fault Tolerant (BFT) state consensus mathematically infallible.
* **[sysinfo](https://github.com/GuillaumeGomez/sysinfo):** For the cross-platform hardware telemetry that allows our Gateway scheduler to optimize workload distribution based on live edge RAM and CPU availability.

## The Polyglot WASI Engines
Thank you to the communities who maintain the `wasm32-wasi` compiled versions of the world's greatest programming languages, allowing our Swarm to be truly polyglot:
* **[CPython](https://github.com/python/cpython)** (Python)
* **[QuickJS](https://bellard.org/quickjs/)** (JavaScript)
* **[Ruby Wasm](https://github.com/ruby/ruby.wasm)** (Ruby)
* **[WasmLabs / PHP](https://wasmlabs.dev/)** (PHP)
* **[SQLite](https://sqlite.org/)** (Database)
* **[Zig](https://ziglang.org/)** (For the flawless, drop-in local cross-compilation toolchain).

---
*Built with gratitude on the edge.* 🐝
