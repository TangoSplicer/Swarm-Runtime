import sys
path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# Logic to probe local ports if mDNS is silent
prober = """
            // Auto-Prober for Termux
            for port in 4000..4005 {
                let _ = p2p_node.swarm.dial(format!("/ip4/127.0.0.1/tcp/{}", port).parse::<libp2p::Multiaddr>().unwrap());
            }
"""
if "p2p_node.subscribe" in content:
    content = content.replace('p2p_node.subscribe("swarm-shard-1")?;', 
                              'p2p_node.subscribe("swarm-shard-1")?;' + prober)

with open(path, "w") as f:
    f.write(content)
