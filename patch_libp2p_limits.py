import os, glob

patched = False
for file in glob.glob("components/**/*.rs", recursive=True):
    with open(file, "r") as f:
        code = f.read()
    
    # Locate the Libp2p request_response initialization
    target = "request_response::Config::default()"
    if target in code and "set_max_request_size" not in code:
        # Inject the 50MB limit dynamically using Rust block expressions
        replacement = "{ let mut cfg = request_response::Config::default(); cfg.set_max_request_size(50 * 1024 * 1024); cfg.set_max_response_size(50 * 1024 * 1024); cfg }"
        code = code.replace(target, replacement)
        
        with open(file, "w") as f:
            f.write(code)
        print(f"✅ Successfully patched Libp2p payload limits in: {file}")
        patched = True

if not patched:
    print("⚠️ Could not find default config. Limits may already be modified.")
