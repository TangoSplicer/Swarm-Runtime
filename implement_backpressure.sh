#!/bin/bash

cd components/swarm-node || exit

# ==========================================
# 1. UPDATE SHARED TYPES
# ==========================================
sed -i 's/UnboundedSender<NodeCommand>/Sender<NodeCommand>/g' src/types.rs

# ==========================================
# 2. UPDATE WORKER BACKPRESSURE
# ==========================================
# Bound the core command channel to 1000 messages
sed -i 's/unbounded_channel::<NodeCommand>()/channel::<NodeCommand>(1000)/g' src/worker.rs

# Bound the P2P internal fetch channel to 100 concurrent requests
sed -i 's/unbounded_channel::<(String, libp2p::PeerId, tokio::sync::oneshot::Sender<Vec<u8>>)>()/channel::<(String, libp2p::PeerId, tokio::sync::oneshot::Sender<Vec<u8>>)>(100)/g' src/worker.rs

# Swap to try_send() to drop packets (load shed) if the buffer is completely full
sed -i 's/\.send(NodeCommand/.try_send(NodeCommand/g' src/worker.rs
sed -i 's/internal_fetch_tx_clone\.send(/internal_fetch_tx_clone.try_send(/g' src/worker.rs

# ==========================================
# 3. UPDATE GATEWAY BACKPRESSURE
# ==========================================
sed -i 's/unbounded_channel::<NodeCommand>()/channel::<NodeCommand>(1000)/g' src/gateway.rs
sed -i 's/\.send(NodeCommand/.try_send(NodeCommand/g' src/gateway.rs
sed -i 's/state\.node_tx\.send/state.node_tx.try_send/g' src/gateway.rs

echo "✅ Asynchronous backpressure and load-shedding implemented."
