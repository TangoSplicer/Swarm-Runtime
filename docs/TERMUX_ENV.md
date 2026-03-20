# Swarm Runtime: Mobile Termux Environment Setup

To ensure exact reproducibility of the Swarm Runtime (v1.0.0) on Android, this document details the exact Termux ecosystem parameters required.

## 1. Core System Packages
```bash
pkg update && pkg upgrade
pkg install rust git build-essential clang openssl-tool python wabt zig
## 2. Environment Variables
​Android imposes strict memory constraints. Add these to your ~/.bashrc:     export RUST_MIN_STACK=8388608
export CARGO_PROFILE_DEV_DEBUG=0
## 3. Build & Execution Protocol
​Always restrict Cargo to a single parallel compilation job to prevent the Android Out-Of-Memory (OOM) killer from terminating the compiler:             cargo build --workspace -j 1
4. Virtual File System (VMFS)
​The Worker nodes utilize the cap-std crate to strictly jail WebAssembly module execution.
​WASI Mount Point: /data (Inside the sandbox)
​Default Host Directory: ./rootfs/data (On the Termux filesystem)
