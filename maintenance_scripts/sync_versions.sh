#!/bin/bash

echo "🚀 Bumping all workspace crates to v0.25.5..."

# Find all Cargo.toml files in the current directory and all subdirectories,
# and overwrite the package version line.
find . -name "Cargo.toml" -type f -exec sed -i 's/^version = .*/version = "0.25.5"/g' {} +

echo "✅ All Cargo.toml files successfully synchronized to v0.25.5."
