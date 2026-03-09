# Swarm Environment Setup

## 1. Single-Device vs Global Mesh Testing
**Local BFT Testing:** If testing on a single device, you must avoid the Shared File System Race Condition and Identity Collisions.
1. Create separate directories: `mkdir worker1_dir && mkdir worker2_dir`.
2. Run workers from isolated directories to preserve separate `.swarm_identity` files.

**Global Mesh Testing (Current):**
To connect a mobile Worker to the Oracle Cloud Gateway, the Worker code must contain the hardcoded explicit Multiaddress of the cloud instance:
`/ip4/<ORACLE_PUBLIC_IP>/tcp/4000/p2p/<GATEWAY_PEER_ID>`

## 2. Polyglot Runtimes (WASI Binaries)
Workers require the following pre-cached engines in their execution directory:
* `python.wasm` (CPython 3.11)
* `qjs.wasm` (QuickJS)
* `lua.wasm` (Lua 5.4 - Patched)
* `ruby.wasm` (Ruby 3.2+ wasip1)
* `php.wasm` (PHP 8.2)

## 3. Toolchains
* **Rust:** `export RUST_MIN_STACK=8388608` (Required for Android compilation).
