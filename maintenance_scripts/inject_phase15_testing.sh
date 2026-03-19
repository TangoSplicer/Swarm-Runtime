#!/bin/bash

echo "🚀 Injecting Phase 15: CI/CD & Security Audit into ROADMAP.md..."

# Use awk to insert the new Phase 15 before the old Phase 15
awk '
/## Phase 15: The Telemetry Control Center/ {
    print "## Phase 15: Enterprise CI/CD, Security & Core Audit - 🚧 NEXT"
    print "- **Automated CI/CD Pipelines:** Implement GitHub Actions workflows for matrix testing across Linux, macOS, and Android (cross-compilation) environments."
    print "- **Comprehensive Integration Testing:** Build automated end-to-end local tests for BFT consensus, CRDT state merging, and Libp2p network partitions."
    print "- **Core Component Hardening:** Audit and strategically advance `synapse` (P2P), `lazarus` (Fault Tolerance/Recovery), and `prism` (AST Compiler/Sharder) to ensure enterprise reliability without over-engineering."
    print "- **Security Scanning:** Integrate `cargo-audit` and dependency vulnerability checks."
    print ""
    print "## Phase 16: The Telemetry Control Center - ⏳ PLANNED"
    next
}
{print}
' docs/ROADMAP.md > docs/ROADMAP.tmp && mv docs/ROADMAP.tmp docs/ROADMAP.md

# Stage, commit, and push the roadmap pivot
git add docs/ROADMAP.md
git commit -m "docs(roadmap): inject Phase 15 CI/CD, Security, and Core Audit

- Prioritized automated GitHub Actions pipelines before V1.0.0
- Added comprehensive BFT/CRDT local testing targets
- Scheduled architectural audit for synapse, lazarus, and prism
- Shifted Telemetry Control Center to Phase 16"
git push origin main

echo "✅ ROADMAP.md successfully updated and pushed to remote!"
