#!/bin/bash

cd components/swarm-node/src || exit

# Find the HA Federation warning line, and delete the single standalone '}' that immediately follows it
awk '
/Operating in local-only mDNS mode/ {
    print
    found=1
    next
}
found == 1 && /^[ \t]*\}[ \t]*$/ {
    # We found the ghost bracket! Skip printing it to delete it.
    found=0 
    next
}
{print}
' worker.rs > worker_fixed.rs

mv worker_fixed.rs worker.rs

echo "✅ Ghost bracket removed."
