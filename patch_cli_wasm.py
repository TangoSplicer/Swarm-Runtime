import os

path = "components/swarm-cli/src/main.rs"
with open(path, "r") as f:
    code = f.read()

replacement = """                "wasm" => {
                    println!("⚙️  Reading raw WebAssembly binary...");
                    let wasm_bytes = fs::read(file).context("Failed to read .wasm file")?;
                    use base64::{Engine as _, engine::general_purpose};
                    let encoded = general_purpose::STANDARD.encode(&wasm_bytes);
                    (encoded, vec![])
                },
                "go" => {"""

if '"go" => {' in code:
    code = code.replace('"go" => {', replacement)
    code = code.replace('php, sqlite, go", lang)', 'php, sqlite, go, wasm", lang)')
    with open(path, "w") as f:
        f.write(code)
    print("✅ CLI successfully patched to accept raw .wasm deployments!")
else:
    print("❌ Could not find target block.")
