import os

path = "components/swarm-cli/src/main.rs"
with open(path, "r") as f:
    code = f.read()

# Replace the empty vectors we added for the Go and Wasm local compilers
target = "(encoded, vec![])"
replacement = '(encoded, vec!["EXECUTE_NATIVE_WASM".to_string()])'

if target in code:
    code = code.replace(target, replacement)
    with open(path, "w") as f:
        f.write(code)
    print("✅ CLI successfully patched to prevent Empty Shard trapping!")
else:
    print("❌ Could not find the empty vectors. Double-check CLI source.")
