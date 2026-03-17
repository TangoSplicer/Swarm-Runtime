#!/bin/bash

echo "🔄 Updating documentation to reflect new directory structure..."

# 1. Update USER_MANUAL.md (Located in docs/)
sed -i 's/deploy app.py/deploy test_payloads\/test_python.py/g' docs/USER_MANUAL.md
sed -i 's/deploy script.js/deploy test_payloads\/test_js.js/g' docs/USER_MANUAL.md
sed -i 's/deploy module.wasm/deploy runtimes\/calculator.wasm/g' docs/USER_MANUAL.md

# 2. Update SETUP.md (Located in docs/)
sed -i 's/in their execution directory:/inside the `runtimes\/` directory:/g' docs/SETUP.md

# 3. Update README.md (Located at root)
sed -i 's/swarm-node deploy` CLI/swarm-node deploy` CLI (e.g., `test_payloads\/test_python.py`)/g' README.md

echo "✅ Documentation paths successfully updated!"
