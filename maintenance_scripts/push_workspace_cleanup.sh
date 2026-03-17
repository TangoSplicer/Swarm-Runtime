#!/bin/bash

# Stage all file moves, deletions, and markdown edits
git add -A

# Commit the organizational and roadmap updates
git commit -m "chore(docs): restructure workspace and inject Phase 13 Hardening

- Cleaned up root directory into docs/, runtimes/, and test_payloads/
- Updated documentation paths to reflect new repository structure
- Injected Phase 13: Core Hardening & Tech Debt Resolution into ROADMAP.md
- Expanded roadmap with Post-V1.0 WORA targets (WASI Component Model)"

# Push to the main branch
git push origin main

echo "✅ Workspace cleanup and new roadmap pushed to remote!"
