with open('components/synapse/tests/chaos_network_test.rs', 'r') as f:
    content = f.read()

# 1. Silence the unused worker_peer warning
content = content.replace('let (worker_peer, mut worker_swarm)', 'let (_worker_peer, mut worker_swarm)')

# 2. Inject a blocking loop that forces the swarm to establish the connection before firing
wait_block = """
    // 🚨 FIX: Poll the swarm to drive the dialing process and wait for the handshake to finish
    loop {
        if let Some(event) = worker_swarm.next().await {
            if let SwarmEvent::ConnectionEstablished { .. } = event {
                break; // Connection is live! Safe to send data.
            }
        }
    }

    let request_id = worker_swarm
        .behaviour_mut()
        .send_request(&gateway_peer,"""

content = content.replace("""    let request_id = worker_swarm
        .behaviour_mut()
        .send_request(&gateway_peer,""", wait_block)

with open('components/synapse/tests/chaos_network_test.rs', 'w') as f:
    f.write(content)

print("✅ Test logic patched to respect async connection handshakes!")
