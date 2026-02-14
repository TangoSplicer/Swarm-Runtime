import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# Tell Serde to ignore the HashSet during serialization
content = content.replace(
    "peers: HashSet<libp2p::PeerId>", 
    "#[serde(skip)]\n    pub peers: HashSet<libp2p::PeerId>"
)

# Also add the peer_count as a serializable field so the dashboard still works
content = content.replace(
    "role: String,",
    "role: String,\n    pub peers_count: usize,"
)

# Update the initialization
content = content.replace(
    'role: "Orchestrator".to_string(),',
    'role: "Orchestrator".to_string(), peers_count: 0,'
)

# Update the count whenever we modify the set
content = content.replace(
    "s.peers.insert(peer_id);",
    "s.peers.insert(peer_id); s.peers_count = s.peers.len();"
)
content = content.replace(
    "s.peers.remove(&peer_id);",
    "s.peers.remove(&peer_id); s.peers_count = s.peers.len();"
)

with open(path, "w") as f:
    f.write(content)
