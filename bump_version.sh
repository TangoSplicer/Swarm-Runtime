#!/bin/bash

# Target the swarm-node Cargo.toml
NODE_TOML="components/swarm-node/Cargo.toml"

# Bump the version directly
sed -i 's/version = "0.22.1"/version = "0.25.5"/g' "$NODE_TOML"

# (Optional) If you have a root workspace Cargo.toml, bump it there too
if [ -f "Cargo.toml" ]; then
    sed -i 's/version = "0.22.1"/version = "0.25.5"/g' Cargo.toml
fi

echo "✅ Version bumped to v0.25.5."
