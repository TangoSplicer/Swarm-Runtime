# Swarm Environment Setup

## 1. Single-Device vs Production Testing
**CRITICAL:** If you are testing the Swarm on a single machine (e.g., multiple Termux tabs), you must avoid the Shared File System Race Condition.
Open `components/swarm-node/src/gateway.rs` and change the `JobState` configuration to `redundancy: 1`. If testing across multiple physical devices, leave it at `redundancy: 2`.

## 2. Polyglot Runtimes (WASI Binaries)
Workers require the following pre-cached engines in their execution directory:
* `python.wasm` (CPython 3.11)
* `qjs.wasm` (QuickJS)
* `lua.wasm` (Lua 5.4 - Patched)
* `ruby.wasm` (Ruby 3.2+ wasip1)
* `php.wasm` (PHP 8.2)
* `sqlite.wasm` (SQLite 3 - Patched)

## 3. Toolchains
* **Rust:** `export RUST_MIN_STACK=8388608` (Required for Android compilation).
* **Zig:** Required in `PATH` for native auto-compilation via the CLI.
