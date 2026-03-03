# Swarm Environment Setup

## 1. Termux (Primary Android Environment)
* **Install:** `pkg install rust git build-essential clang openssl-tool python wabt zig`
* **Crucial Build Flags:** Due to Android's memory limits, you must throttle the Rust compiler:
  ```bash
  export RUST_MIN_STACK=8388608
  CARGO_PROFILE_DEV_DEBUG=0 cargo build --workspace -j 1      2. Polyglot Runtimes (WASI Binaries)
​The Workers require the underlying WebAssembly engines to be cached in their local execution directory to handle Interpreted Languages. Ensure the following files are downloaded into your project root:
​python.wasm (CPython 3.11)
​qjs.wasm (QuickJS)
​lua.wasm (Lua 5.4 - Patched for snapshot_preview1)
​ruby.wasm (Ruby 3.2+ wasip1 CLI Command Module)
​php.wasm (PHP 8.2 Slim)
​sqlite.wasm (SQLite 3 - Patched for snapshot_preview1)
​3. Compiled Target Toolchains
​Zig: The CLI requires the zig compiler in your PATH to dynamically run zig build-exe before network transfer.
​Wabt: Required if compiling .wat (WebAssembly Text format) files directly.
​4. Cross-Platform Validation
​Linux/WSL: Identical to Termux. Standard cargo build works natively.
