# Acknowledgements

The Swarm Runtime is built on the shoulders of absolute giants. This architecture would not be possible without the incredible engineering behind the following open-source projects and platforms:

## Core Crates & Technologies
* **[rust-libp2p](https://github.com/libp2p/rust-libp2p):** The backbone of the Swarm. Provided the Kademlia DHT, Gossipsub routing, Noise encryption, and Yamux multiplexing that allowed us to bypass corporate NATs and establish a resilient global mesh.
* **[wasmi](https://github.com/wasmi-labs/wasmi):** The execution engine. Allowed us to securely sandbox and run untrusted WebAssembly payloads natively on ARM64 mobile processors.
* **[axum](https://github.com/tokio-rs/axum) & [tokio](https://github.com/tokio-rs/tokio):** The asynchronous heart of the Gateway. Provided the ultra-fast HTTP routing and non-blocking I/O needed to orchestrate the mesh.
* **[cap-std](https://github.com/bytecodealliance/cap-std):** The security warden. Provided the capability-based file system sandboxing necessary for the Virtual File System (VMFS).

## Platforms & Infrastructure
* **[Termux](https://termux.dev/en/):** The ultimate Android terminal emulator. Termux proved that physical mobile phones are fully capable Linux supercomputers, allowing us to compile and run complex Rust distributed systems natively from our pockets.
* **[Oracle Cloud Infrastructure (OCI)](https://www.oracle.com/cloud/):** The `E2.1.Micro` Always Free tier provided the perfect lightweight, public-facing anchor node to bootstrap the Global Mesh over the WAN.
