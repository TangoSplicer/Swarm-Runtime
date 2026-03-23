with open('components/swarm-cli/Cargo.toml', 'r') as f:
    content = f.read()

old_line = 'reqwest = { version = "0.11", features = ["blocking", "json"] }'
new_line = 'reqwest = { version = "0.11", default-features = false, features = ["rustls-tls", "blocking", "json"] }'

with open('components/swarm-cli/Cargo.toml', 'w') as f:
    f.write(content.replace(old_line, new_line))

print("✅ OpenSSL eradicated from swarm-cli!")
