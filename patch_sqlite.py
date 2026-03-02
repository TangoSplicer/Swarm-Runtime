import os

# 1. Update Judge
judge_path = "components/judge/src/lib.rs"
with open(judge_path, "r") as f:
    code = f.read()

target = """            } else if polyglot_id == "POLYGLOT:PHP" {
                target_file = "app.php";
                wasi_args = vec!["php".to_string(), "/data/app.php".to_string()];
            }"""
replacement = """            } else if polyglot_id == "POLYGLOT:PHP" {
                target_file = "app.php";
                wasi_args = vec!["php".to_string(), "/data/app.php".to_string()];
            } else if polyglot_id == "POLYGLOT:SQLITE" {
                target_file = "app.sql";
                wasi_args = vec!["sqlite3".to_string(), "/data/swarm.db".to_string(), ".read /data/app.sql".to_string()];
            }"""
if target in code:
    code = code.replace(target, replacement)
    with open(judge_path, "w") as f: f.write(code)

# 2. Update Worker
worker_path = "components/swarm-node/src/worker.rs"
with open(worker_path, "r") as f:
    code = f.read()

target = """                                                    "POLYGLOT:PHP" => Ok(std::fs::read("php.wasm").unwrap_or_default()),"""
replacement = """                                                    "POLYGLOT:PHP" => Ok(std::fs::read("php.wasm").unwrap_or_default()),
                                                    "POLYGLOT:SQLITE" => Ok(std::fs::read("sqlite.wasm").unwrap_or_default()),"""
if target in code:
    code = code.replace(target, replacement)
    with open(worker_path, "w") as f: f.write(code)

# 3. Update CLI
cli_path = "components/swarm-cli/src/main.rs"
with open(cli_path, "r") as f:
    code = f.read()

target = """                "php" => "POLYGLOT:PHP",
                _ => anyhow::bail!("Unsupported language: {}. Currently supported: python, js, lua, ruby, php", lang),"""
replacement = """                "php" => "POLYGLOT:PHP",
                "sqlite" | "sql" => "POLYGLOT:SQLITE",
                _ => anyhow::bail!("Unsupported language: {}. Currently supported: python, js, lua, ruby, php, sqlite", lang),"""
if target in code:
    code = code.replace(target, replacement)
    with open(cli_path, "w") as f: f.write(code)

print("✅ SQLite Polyglot routing injected.")
