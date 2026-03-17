#!/bin/bash

echo "📝 Updating ROADMAP.md to mark Phase 11 as COMPLETED..."

# Change Phase 11 from NEXT/PLANNED to COMPLETED
sed -i 's/## Phase 11: Polyglot & Integration Verification - 🚧 NEXT/## Phase 11: Polyglot & Integration Verification - ✅ COMPLETED/g' docs/ROADMAP.md
sed -i 's/## Phase 11: Polyglot & Integration Verification - ⏳ PLANNED/## Phase 11: Polyglot & Integration Verification - ✅ COMPLETED/g' docs/ROADMAP.md

# Update the bullet points to past tense to reflect the audit
sed -i 's/- \*\*Language Sandbox Testing:\*\* Systematically verify execution/- \*\*Language Sandbox Testing:\*\* Verified execution/g' docs/ROADMAP.md
sed -i 's/- \*\*Failure Condition Audits:\*\* Simulate edge-case network drops/- \*\*Failure Condition Audits:\*\* Simulated edge-case network drops/g' docs/ROADMAP.md

echo "✅ ROADMAP.md successfully updated!"
