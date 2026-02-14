#!/bin/bash

# 1. Update all Cargo.toml versions to 0.8.0
echo "versioning 0.8.0..."
find . -name "Cargo.toml" -exec sed -i 's/^version = "0.7.0"/version = "0.8.0"/' {} +

# 2. Stage changes
git add .

# 3. Commit
git commit -m "feat: implement v0.8.0 distributed parallel reduction and auto-discovery prober"

# 4. Push to GitHub (main branch)
echo "🚀 Pushing to GitHub..."
git push origin main

# 5. Create and push tag
echo "🏷️ Tagging v0.8.0..."
git tag -a v0.8.0 -m "Release v0.8.0: Sharding and Map-Reduce functional"
git push origin v0.8.0

echo "✅ Release complete! Your swarm is now at v0.8.0 on GitHub."
