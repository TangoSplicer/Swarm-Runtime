#!/bin/bash

cd components/swarm-node/src || exit

# 1. Escape the curly braces in gateway.rs for the format! macro
sed -i 's/SYNC_STATE:{\\"k\\":\\"{}\\",\\"v\\":\\"{}\\"}/SYNC_STATE:{{\\"k\\":\\"{}\\",\\"v\\":\\"{}\\"}}/g' gateway.rs

# 2. Add the GatewaySync ignore arm to worker.rs
sed -i 's/NodeCommand::Broadcast(msg) => { let _ = p2p_node.publish_to_topic("swarm-control-plane", msg); },/NodeCommand::Broadcast(msg) => { let _ = p2p_node.publish_to_topic("swarm-control-plane", msg); },\n                        NodeCommand::GatewaySync(_) => {},/g' worker.rs

echo "✅ Compiler errors patched."
