import os

# 1. Patch the CLI
cli_path = "components/swarm-cli/src/main.rs"
with open(cli_path, "r") as f:
    code = f.read()

if '"php" => "POLYGLOT:PHP"' not in code:
    code = code.replace(
        '"ruby" | "rb" => "POLYGLOT:RUBY",',
        '"ruby" | "rb" => "POLYGLOT:RUBY",\n                "php" => "POLYGLOT:PHP",'
    )
    code = code.replace(
        'python, js, lua, ruby"',
        'python, js, lua, ruby, php"'
    )
    with open(cli_path, "w") as f:
        f.write(code)
    print("✅ CLI successfully patched for PHP!")

# 2. Patch the Worker
worker_path = "components/swarm-node/src/worker.rs"
with open(worker_path, "r") as f:
    w_code = f.read()

if '"POLYGLOT:PHP"' not in w_code:
    w_code = w_code.replace(
        '"POLYGLOT:RUBY" => Ok(std::fs::read("ruby.wasm").unwrap_or_default()),',
        '"POLYGLOT:RUBY" => Ok(std::fs::read("ruby.wasm").unwrap_or_default()),\n                                                    "POLYGLOT:PHP" => Ok(std::fs::read("php.wasm").unwrap_or_default()),'
    )
    with open(worker_path, "w") as f:
        f.write(w_code)
    print("✅ Worker successfully patched for PHP!")
