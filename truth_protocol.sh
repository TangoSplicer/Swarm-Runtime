#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "\n${YELLOW}[INIT] Starting Truth Protocol...${NC}"
echo "=========================================="

# Function to run a test suite
run_check() {
    COMPONENT=$1
    DESCRIPTION=$2
    
    echo -e "\n${YELLOW}[CHECKING] ${DESCRIPTION} (${COMPONENT})...${NC}"
    
    # We use -j 1 to run tests sequentially, preventing file locking race conditions on Termux
    if cargo test -p $COMPONENT --quiet -- --test-threads=1; then
        echo -e "${GREEN}[PASS] ${COMPONENT} integrity verified.${NC}"
        return 0
    else
        echo -e "${RED}[FAIL] ${COMPONENT} failed verification.${NC}"
        return 1
    fi
}

# 1. Verify Prism Compiler
run_check "prism" "Compiler Logic & State Detection"
PRISM_STATUS=$?

# 2. Verify Synapse Protocol
run_check "synapse" "P2P Identity & Networking"
SYNAPSE_STATUS=$?

# 3. Verify Judge Hypervisor
run_check "judge" "Wasm Runtime & Security Sandbox"
JUDGE_STATUS=$?

# 4. Verify Lazarus Daemon
run_check "lazarus" "Watchdog & Auto-Recovery"
LAZARUS_STATUS=$?

echo -e "\n=========================================="
echo -e "${YELLOW}[REPORT] Final System Status:${NC}"

if [ $PRISM_STATUS -eq 0 ] && [ $SYNAPSE_STATUS -eq 0 ] && [ $JUDGE_STATUS -eq 0 ] && [ $LAZARUS_STATUS -eq 0 ]; then
    echo -e "${GREEN}ALL SYSTEMS NOMINAL. READY FOR DEPLOYMENT.${NC}"
    exit 0
else
    echo -e "${RED}CRITICAL FAILURES DETECTED. DO NOT DEPLOY.${NC}"
    exit 1
fi
