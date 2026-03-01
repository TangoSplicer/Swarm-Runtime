import os

path = "components/judge/src/lib.rs"
with open(path, "r") as f:
    code = f.read()

# Increase the fuel limit from 5 Million to 50 Billion
target = "store.add_fuel(5_000_000)"
replacement = "store.add_fuel(50_000_000_000)"

if target in code:
    code = code.replace(target, replacement)
    with open(path, "w") as f:
        f.write(code)
    print("✅ Wasmi fuel limit upgraded to 50 Billion operations!")
else:
    print("⚠️ Target not found. Check if already patched.")
