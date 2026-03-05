# Swarm Runtime: Mobile Termux Environment Setup

To ensure exact reproducibility of the Swarm Runtime on Android, this document details the exact Termux ecosystem parameters required.

## 1. Core System Packages
```bash
pkg update && pkg upgrade
pkg install rust git build-essential clang openssl-tool python wabt zig
2. Environment Variables
​Android imposes strict memory and stack size limits on terminal processes. The Rust compiler will hit a stack overflow during deep LLVM passes (especially while compiling libp2p or wasmi) without the following flags:                                 # Add this to your ~/.bashrc or ~/.zshrc
# export RUST_MIN_STACK=8388608
# export CARGO_PROFILE_DEV_DEBUG=0
3. Build & Execution Protocol
​Always restrict cargo to a single parallel job to prevent the Android Out-Of-Memory (OOM) killer from destroying the compiler:                                                            cargo build --workspace -j 1
4. Virtual File System Mapping
​The Worker nodes utilize the cap-std crate to strictly jail WebAssembly module execution.
​Host Directory: ~/clones/swarm-runtime/rootfs/data
​WASI Mount Point: /data
