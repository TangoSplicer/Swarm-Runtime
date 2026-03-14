#!/bin/bash

cd components/swarm-node || exit

# ==========================================
# 1. CLEANUP WARNINGS
# ==========================================
# Silence the unused struct field warning
sed -i 's/breakdown: Vec<(u32, i32)>,/_breakdown: Vec<(u32, i32)>,/g' src/main.rs

# ==========================================
# 2. CLI NETWORK RESILIENCE 
# ==========================================
# Enforce a strict 15-second timeout on all CLI HTTP requests to handle packet loss
sed -i 's/let client = reqwest::Client::new();/let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();/g' src/main.rs

# ==========================================
# 3. GATEWAY MESH TIMEOUTS
# ==========================================
# Wrap the Gateway's internal P2P oneshot receiver in a 10-second Tokio timeout
sed -i 's/if let Ok(Some(bytes)) = rx.await {/if let Ok(Ok(Some(bytes))) = tokio::time::timeout(std::time::Duration::from_secs(10), rx).await {/g' src/gateway.rs

echo "✅ Strict network timeouts and resilience logic applied."
