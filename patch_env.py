import os

path = "components/judge/src/lib.rs"
with open(path, "r") as f:
    code = f.read()

# Fix the method chaining by unpacking the Result between each call
target = """.env("PYTHONPATH", "/python-wasi.zip") // DIRECT ZIP IMPORT
            .env("PYTHONHOME", "/")
            .map_err(|e| anyhow!("WASI env error: {}", e))?"""

replacement = """.env("PYTHONPATH", "/python-wasi.zip")
            .map_err(|e| anyhow!("WASI env error: {}", e))?
            .env("PYTHONHOME", "/")
            .map_err(|e| anyhow!("WASI env error: {}", e))?"""

if target in code:
    code = code.replace(target, replacement)
    with open(path, "w") as f:
        f.write(code)
    print("✅ Judge API signatures correctly mapped!")
else:
    print("⚠️ Target not found.")
