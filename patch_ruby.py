import os

# 1. Update Judge
judge_path = "components/judge/src/lib.rs"
with open(judge_path, "r") as f:
    code = f.read()

target = """            } else if polyglot_id == "POLYGLOT:LUA" {
                target_file = "app.lua";
                wasi_args = vec!["lua".to_string(), "/data/app.lua".to_string()];
            }"""
replacement = """            } else if polyglot_id == "POLYGLOT:LUA" {
                target_file = "app.lua";
                wasi_args = vec!["lua".to_string(), "/data/app.lua".to_string()];
            } else if polyglot_id == "POLYGLOT:RUBY" {
                target_file = "app.rb";
                wasi_args = vec!["ruby".to_string(), "/data/app.rb".to_string()];
            }"""
if target in code:
    code = code.replace(target, replacement)
    with open(judge_path, "w") as f: f.write(code)

# 2. Update Worker
worker_path = "components/swarm-node/src/worker.rs"
with open(worker_path, "r") as f:
    code = f.read()

target = """                                                    "POLYGLOT:LUA" => Ok(std::fs::read("lua.wasm").unwrap_or_default()),"""
replacement = """                                                    "POLYGLOT:LUA" => Ok(std::fs::read("lua.wasm").unwrap_or_default()),
                                                    "POLYGLOT:RUBY" => Ok(std::fs::read("ruby.wasm").unwrap_or_default()),"""
if target in code:
    code = code.replace(target, replacement)
    with open(worker_path, "w") as f: f.write(code)

# 3. Update CLI
cli_path = "components/swarm-cli/src/main.rs"
with open(cli_path, "r") as f:
    code = f.read()

target = """                "lua" => "POLYGLOT:LUA",
                _ => anyhow::bail!("Unsupported language: {}. Currently supported: python, js, lua", lang),"""
replacement = """                "lua" => "POLYGLOT:LUA",
                "ruby" | "rb" => "POLYGLOT:RUBY",
                _ => anyhow::bail!("Unsupported language: {}. Currently supported: python, js, lua, ruby", lang),"""
if target in code:
    code = code.replace(target, replacement)
    with open(cli_path, "w") as f: f.write(code)

print("✅ Ruby Polyglot routing injected.")
