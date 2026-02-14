import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# 1. Add HashSet import
content = "use std::collections::HashSet;\n" + content

# 2. Update SwarmStatus to use a Set for counting
content = content.replace("peers_count: usize", "peers: HashSet<libp2p::PeerId>")
content = content.replace("peers_count: 0", "peers: HashSet::new()")

# 3. Update the discovery logic to insert into the Set
content = content.replace(
    "s.peers_count += 1;", 
    "s.peers.insert(peer_id);"
)
content = content.replace(
    "s.peers_count = s.peers_count.saturating_sub(1);", 
    "s.peers.remove(&peer_id);"
)

# 4. Update the range splitting to use the set length
content = content.replace(
    "let peer_count = state.stats.lock().await.peers_count.max(1);",
    "let peer_count = state.stats.lock().await.peers.len().max(1);"
)

with open(path, "w") as f:
    f.write(content)
