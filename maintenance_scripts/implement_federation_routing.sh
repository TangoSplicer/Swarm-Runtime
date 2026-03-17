#!/bin/bash

cd components/swarm-node || exit

# ==========================================
# 1. UPDATE CLI TO ACCEPT MULTIPLE GATEWAYS
# ==========================================
# Change the default gateway arg to allow comma-separated lists
sed -i 's/gateway: String,/gateways: String,/g' src/main.rs

# Update the Deploy command routing logic to try multiple gateways
sed -i 's/let url = format!("{}/let gateways: Vec<&str> = gateways.split(',').map(|s| s.trim()).collect();\n            let mut success = false;\n            for gw in gateways {\n                let url = format!("{}/g' src/main.rs

sed -i 's/if res.status().is_success() {/if res.status().is_success() {\n                    println!("✅ Deployment Successful via {}!", gw);\n                    success = true;\n                    break;\n                }/g' src/main.rs

sed -i 's/println!("❌ Deployment Failed/println!("⚠️ Failed on {} (Status: {})", gw, res.status());\n            }\n            if !success {\n                println!("❌ Deployment Failed across all federated Gateways.");/g' src/main.rs

# ==========================================
# 2. UPDATE WORKER TO DIAL MULTIPLE GATEWAYS
# ==========================================
# Remove the old hardcoded Oracle IP dial logic
sed -i '/let cloud_gateway: libp2p::Multiaddr/,/Dialing Oracle Cloud Gateway...");/d' src/worker.rs

# Inject the High Availability (HA) Array Dialer
sed -i '/p2p_node.subscribe("swarm-control-plane")?;/a \
\
    // PHASE 12: High Availability Gateway Federation\
    let bootstrap_nodes = vec![\
        "/ip4/145.241.192.79/tcp/4000/p2p/12D3KooWR6zCohbghjfRyUriggGyq6HKWJxapyziq2icYgXGeEiK", // Oracle Primary\
        // "/ip4/<AWS_IP>/tcp/4000/p2p/<AWS_PEER_ID>", // AWS Secondary (Placeholder)\
    ];\
\
    let mut dialed_any = false;\
    for node_str in bootstrap_nodes {\
        if let Ok(multiaddr) = node_str.parse::<libp2p::Multiaddr>() {\
            if p2p_node.swarm.dial(multiaddr.clone()).is_ok() {\
                println!("🚀 Dialing Federated Gateway: {}", node_str);\
                dialed_any = true;\
            }\
        }\
    }\
    if !dialed_any {\
        println!("⚠️ WARNING: Could not dial any federated Gateways. Operating in local-only mDNS mode.");\
    }' src/worker.rs

echo "✅ HA Federation Routing implemented for CLI and Workers."
