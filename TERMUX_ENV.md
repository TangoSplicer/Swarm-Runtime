# Swarm Runtime: Mobile Termux Environment Setup

To ensure exact reproducibility of the Swarm Runtime (v0.23.0) on Android, this document details the exact Termux ecosystem parameters, packages, and constraints required.

## 1. Core System Packages
Ensure your Termux environment is fully up-to-date and contains the required build toolchains:
```bash
pkg update && pkg upgrade
pkg install rust git build-essential clang openssl-tool python wabt zig
2. Environment Variables
​Android imposes strict memory and stack size limits on terminal processes. The Rust compiler will hit a stack overflow during deep LLVM passes (especially while compiling complex crates like libp2p or wasmi) without the following environment variables.
​Add these to your ~/.bashrc or ~/.zshrc:                      export RUST_MIN_STACK=8388608
export CARGO_PROFILE_DEV_DEBUG=0
3. Build & Execution Protocol
​Mobile devices have limited RAM compared to desktop development environments. Always restrict Cargo to a single parallel compilation job to prevent the Android Out-Of-Memory (OOM) killer from terminating the compiler:                               cargo build --workspace -j 1
4. Virtual File System (VMFS) Mapping & BFT Isolation
​The Worker nodes utilize the cap-std crate to strictly jail WebAssembly module execution.
​WASI Mount Point: /data (Inside the sandbox)
​Default Host Directory: ./rootfs/data (On the Termux filesystem)
​⚠️ CRITICAL FOR LOCAL TESTING: If you are running multiple Workers on the same Termux instance to test Byzantine Fault Tolerance (BFT) and P2P State Synchronization, you cannot share the same host directory.
You must physically isolate the execution paths so they generate their own .state files without read/write collisions:      mkdir worker1_dir && mkdir worker2_dir
# Run Worker 1 from inside worker1_dir
# Run Worker 2 from inside worker2_dir
