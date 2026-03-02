import os

path = "components/swarm-node/src/worker.rs"
with open(path, "r") as f:
    code = f.read()

# Replace the double Lua entry with a single one
target = """                                                    "POLYGLOT:LUA" => Ok(std::fs::read("lua.wasm").unwrap_or_default()),
                                                    "POLYGLOT:LUA" => Ok(std::fs::read("lua.wasm").unwrap_or_default()),"""
replacement = """                                                    "POLYGLOT:LUA" => Ok(std::fs::read("lua.wasm").unwrap_or_default()),"""

if target in code:
    code = code.replace(target, replacement)
    with open(path, "w") as f:
        f.write(code)
    print("✅ Duplicate Lua match arm removed.")
