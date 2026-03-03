import os

path = "components/swarm-cli/src/main.rs"
with open(path, "r") as f:
    code = f.read()

replacement = """                "zig" => {
                    println!("⚙️  Locally compiling Zig source natively to WASIp1...");
                    let temp_wasm = format!("{}.wasm", file.replace(".zig", "").replace("/", "_"));
                    
                    let status = std::process::Command::new("zig")
                        .args([
                            "build-exe", file, 
                            "-target", "wasm32-wasi", 
                            "-O", "ReleaseSmall", 
                            &format!("-femit-bin={}", temp_wasm)
                        ])
                        .status()
                        .context("Failed to execute zig compiler. Is Zig installed?")?;

                    if !status.success() {
                        anyhow::bail!("❌ Zig compilation failed!");
                    }

                    let wasm_bytes = std::fs::read(&temp_wasm).context("Failed to read compiled wasm")?;
                    let _ = std::fs::remove_file(&temp_wasm); // Cleanup binary
                    let _ = std::fs::remove_file(format!("{}.o", temp_wasm)).unwrap_or_default(); // Cleanup object file
                    
                    use base64::{Engine as _, engine::general_purpose};
                    let encoded = general_purpose::STANDARD.encode(&wasm_bytes);
                    (encoded, vec!["EXECUTE_NATIVE_WASM".to_string()])
                },
                "wasm" => {"""

if '"wasm" => {' in code:
    code = code.replace('"wasm" => {', replacement)
    code = code.replace('php, sqlite, go, wasm", lang)', 'php, sqlite, go, wasm, zig", lang)')
    with open(path, "w") as f:
        f.write(code)
    print("✅ CLI successfully patched for Local Zig Compilation!")
else:
    print("❌ Could not find target block.")
