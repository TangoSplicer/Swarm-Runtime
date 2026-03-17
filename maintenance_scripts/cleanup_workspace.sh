#!/bin/bash

echo "🧹 Cleaning up Swarm Runtime root directory..."

# 1. Create organized directories
mkdir -p docs runtimes test_payloads maintenance_scripts

# 2. Move all generated bash patch/maintenance scripts
mv *.sh maintenance_scripts/ 2>/dev/null

# 3. Move all pre-compiled WebAssembly engines
mv *.wasm runtimes/ 2>/dev/null

# 4. Move all polyglot test scripts and example contracts
mv test_* test_payloads/ 2>/dev/null
mv stateful-counter test_payloads/ 2>/dev/null

# 5. Move architecture and reference documentation (keeping README and CHANGELOG at root)
mv ACKNOWLEDGEMENTS.md ALIGNMENT.md HIGH_IMPACT_ENTERPRISE.md HighImpactLowImplementation_USE_CASES.md INTEGRATION_GUIDE.md LOCAL_TESTING.md ROADMAP.md SETUP.md TECHNICAL.md TECHNICAL_SPEC.md TERMUX_ENV.md USER_MANUAL.md docs/ 2>/dev/null

echo "✅ Workspace organized!"
echo "📂 Use 'ls -l' to view your new clean directory structure."
