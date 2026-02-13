##🛠 Technical Architecture

###Networking: 
libp2p using GossipSub for task broadcasting and mDNS for local discovery.

###Consensus: 
A custom Raft-lite implementation in the synapse crate for leader election and heartbeats.

###Runtime: 
Wasmer engine with custom Host Calls (db_set, db_get).

###State: 
sled high-performance embedded KV-store.