import os

path = "components/swarm-cli/src/main.rs"
with open(path, "r") as f:
    code = f.read()

target = """            let code = fs::read_to_string(file)
                .with_context(|| format!("Failed to read file: {}", file))?;

            let identifier = match lang.to_lowercase().as_str() {
                "python" => "POLYGLOT:PYTHON",
                "js" | "javascript" => "POLYGLOT:JS",
                "lua" => "POLYGLOT:LUA",
                "ruby" | "rb" => "POLYGLOT:RUBY",
                "php" => "POLYGLOT:PHP",
                "sqlite" | "sql" => "POLYGLOT:SQLITE",
                _ => anyhow::bail!("Unsupported language: {}. Currently supported: python, js, lua, ruby, php, sqlite", lang),
            };

            let payload = DeployPayload {
                wasm_base64: identifier.to_string(),
                dataset: vec![code],
            };"""

replacement = """            let (wasm_base64, dataset) = match lang.to_lowercase().as_str() {
                "go" => {
                    println!("⚙️  Locally compiling Go source natively to WASIp1...");
                    let temp_wasm = format!("{}.wasm", file.replace(".go", "").replace("/", "_"));
                    let status = std::process::Command::new("go")
                        .env("GOOS", "wasip1")
                        .env("GOARCH", "wasm")
                        .args(["build", "-ldflags=-s -w", "-o", &temp_wasm, file])
                        .status()
                        .context("Failed to execute go compiler. Is Go installed?")?;

                    if !status.success() {
                        anyhow::bail!("❌ Go compilation failed!");
                    }

                    let wasm_bytes = fs::read(&temp_wasm).context("Failed to read compiled wasm")?;
                    let _ = fs::remove_file(&temp_wasm); // Cleanup

                    use base64::{Engine as _, engine::general_purpose};
                    let encoded = general_purpose::STANDARD.encode(&wasm_bytes);
                    (encoded, vec![])
                },
                "python" | "js" | "javascript" | "lua" | "ruby" | "rb" | "php" | "sqlite" | "sql" => {
                    let code = fs::read_to_string(file)
                        .with_context(|| format!("Failed to read file: {}", file))?;
                        
                    let identifier = match lang.to_lowercase().as_str() {
                        "python" => "POLYGLOT:PYTHON",
                        "js" | "javascript" => "POLYGLOT:JS",
                        "lua" => "POLYGLOT:LUA",
                        "ruby" | "rb" => "POLYGLOT:RUBY",
                        "php" => "POLYGLOT:PHP",
                        "sqlite" | "sql" => "POLYGLOT:SQLITE",
                        _ => unreachable!(),
                    };
                    (identifier.to_string(), vec![code])
                },
                _ => anyhow::bail!("Unsupported language: {}. Currently supported: python, js, lua, ruby, php, sqlite, go", lang),
            };

            let payload = DeployPayload {
                wasm_base64,
                dataset,
            };"""

if target in code:
    code = code.replace(target, replacement)
    with open(path, "w") as f:
        f.write(code)
    print("✅ CLI successfully patched for Local Go Compilation!")
else:
    print("❌ Target code block not found. Double-check indentation.")
