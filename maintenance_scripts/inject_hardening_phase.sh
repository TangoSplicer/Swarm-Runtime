#!/bin/bash

echo "🚀 Injecting Pre-V1.0 Audit Resolution into ROADMAP.md..."

# Shift existing phases down by 1
sed -i 's/Phase 19/Phase 20/g' docs/ROADMAP.md
sed -i 's/Phase 18/Phase 19/g' docs/ROADMAP.md
sed -i 's/Phase 17/Phase 18/g' docs/ROADMAP.md
sed -i 's/Phase 16/Phase 17/g' docs/ROADMAP.md
sed -i 's/Phase 15/Phase 16/g' docs/ROADMAP.md
sed -i 's/Phase 14/Phase 15/g' docs/ROADMAP.md
sed -i 's/Phase 13/Phase 14/g' docs/ROADMAP.md

# Insert the new Phase 13 before Phase 14
awk '
/## Phase 14: Multi-Shard/ {
    print "## Phase 13: Core Hardening & Tech Debt Resolution - 🚧 NEXT"
    print "- **Payload & Path Security:** Enforce cryptographic signature verification on Edge Workers and implement Cap-Std VMFS path sanitization to prevent directory traversal."
    print "- **Tokio Deadlock Eradication:** Refactor the Gateway DashMap scheduling loops to strictly enforce the clone-and-release pattern."
    print "- **WASI Standardization:** Eliminate byte-concatenation state injection in favor of secure, sandboxed POSIX file mounts."
    print "- **Algorithm & Network Optimization:** Eradicate O(n²) shard detection bottlenecks and implement adaptive Gossipsub telemetry backoff to prevent network flooding."
    print "- **API & UX Polish:** Connect the Axum router to the frontend dashboard, enforce strict HTTP payload validation, and dynamically sync version strings."
    print ""
}
{print}
' docs/ROADMAP.md > docs/ROADMAP.tmp && mv docs/ROADMAP.tmp docs/ROADMAP.md

echo "✅ ROADMAP.md successfully updated with Phase 13 Core Hardening targets!"
