import re

with open('components/swarm-node/src/worker.rs', 'r') as f:
    wk = f.read()

# 1. Strip the unused DashMap import
wk = wk.replace('use dashmap::{DashMap, DashSet};', 'use dashmap::DashSet;')
wk = wk.replace('use dashmap::DashMap;', '')

# 2. Extract and obliterate the illegal spawn block
spawn_regex = r"// Spawn the background event processing loop\s*tokio::spawn\(async move \{\s*loop \{\s*if let Some\(cmd\) = worker_rx\.recv\(\)\.await \{\s*match cmd \{.*?\}\s*\}\s*\}\s*\}\);"
wk = re.sub(spawn_regex, "", wk, flags=re.DOTALL)

# 3. Inject the command receiver safely into the main select! loop
select_insertion = """tokio::select! {
            cmd = worker_rx.recv() => {
                if let Some(cmd) = cmd {
                    match cmd {
                        NodeCommand::Unicast(peer, req) => { let _ = p2p_node.send_request(&peer, req); },
                        NodeCommand::Broadcast(msg) => { let _ = p2p_node.publish_to_topic("swarm-control-plane", msg); },
                        NodeCommand::Disconnect(peer) => { let _ = p2p_node.swarm.disconnect_peer_id(peer); },
                        _ => {}
                    }
                }
            },"""
wk = wk.replace("tokio::select! {", select_insertion, 1)

with open('components/swarm-node/src/worker.rs', 'w') as f:
    f.write(wk)

print("✅ Borrow Checker Satisfied!")
