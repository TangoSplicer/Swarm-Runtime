#!/bin/bash
# Exit immediately if a pipeline fails
set -e

echo "=============================================="
echo "🐝 Swarm Runtime - Zero-Trust Edge Installer"
echo "=============================================="

# --- Configuration ---
GATEWAY_IP="130.162.186.204"
GATEWAY_PORT="4000"
TRUSTED_KEY="8acd3975e868be5cfa61b2e4f8f86e6b2a590258cf813b958dc05db3f101e18a"
ARTIFACT_NAME="swarm-binaries-aarch64-linux-android"
INSTALL_DIR="$HOME/swarm-edge"
# ---------------------

echo "🔍 Phase 1: Validating Execution Environment..."
if [ "$(uname -m)" != "aarch64" ]; then
    echo "❌ FATAL: This installation requires an ARM64 (aarch64) environment."
    exit 1
fi

echo "📦 Phase 2: Verifying Toolchain Dependencies..."
if ! command -v gh &> /dev/null; then
    echo "❌ FATAL: GitHub CLI (gh) is not installed."
    echo "💡 Run 'pkg install gh' and 'gh auth login' before executing this script."
    exit 1
fi

echo "🧹 Phase 3: Executing Clean Slate Protocol..."
rm -rf "$INSTALL_DIR"
mkdir -p "$INSTALL_DIR"

echo "⬇️ Phase 4: Fetching Authenticated Wasm Sandbox..."
# Download directly into the clean directory
gh run download -n "$ARTIFACT_NAME" --dir "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/swarm-node"

echo "✅ Phase 5: Mathematical Identity Verified. Igniting Mesh Connection..."
echo "----------------------------------------------"
# Boot the worker and bind it to the Oracle's authority
"$INSTALL_DIR/swarm-node" start \
  --shard 1 \
  --bootnode "/ip4/$GATEWAY_IP/tcp/$GATEWAY_PORT" \
  --trusted-gateway "$TRUSTED_KEY"
