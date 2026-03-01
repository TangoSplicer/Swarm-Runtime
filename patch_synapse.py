import os

path = "components/synapse/src/lib.rs"
with open(path, "r") as f:
    code = f.read()

target = "request_response::Config::default()"
replacement = """{
            let mut cfg = request_response::Config::default();
            cfg.set_max_request_size(50_000_000);
            cfg.set_max_response_size(50_000_000);
            cfg.set_request_timeout(std::time::Duration::from_secs(300));
            cfg
        }"""

if target in code:
    code = code.replace(target, replacement)
    with open(path, "w") as f:
        f.write(code)
    print("✅ Synapse P2P limits lifted to 50MB!")
else:
    print("⚠️ Target not found. Check if already patched.")
