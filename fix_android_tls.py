import os
import re

print("🔍 Scanning for OpenSSL/reqwest dependencies...")

for root, dirs, files in os.walk('.'):
    if 'Cargo.toml' in files:
        path = os.path.join(root, 'Cargo.toml')
        with open(path, 'r') as f:
            content = f.read()
        
        # If reqwest is in the file but doesn't have rustls configured yet
        if 'reqwest' in content and 'rustls-tls' not in content:
            # Swap simple version strings: reqwest = "0.11"
            content = re.sub(
                r'reqwest\s*=\s*"([^"]+)"', 
                r'reqwest = { version = "\1", default-features = false, features = ["rustls-tls"] }', 
                content
            )
            # Swap inline dictionary declarations: reqwest = { version = "0.11" }
            content = re.sub(
                r'reqwest\s*=\s*\{\s*version\s*=\s*"([^"]+)"\s*\}', 
                r'reqwest = { version = "\1", default-features = false, features = ["rustls-tls"] }', 
                content
            )
            
            with open(path, 'w') as f:
                f.write(content)
            print(f"✅ Patched {path} to use pure-Rust rustls!")

print("🚀 Patch complete.")
