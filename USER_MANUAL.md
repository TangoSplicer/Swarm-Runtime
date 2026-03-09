# Swarm Runtime: Developer User Manual (v0.25.0)

## 1. Starting the Mesh & Cryptographic Identity
As of v0.25.0, every node dynamically generates a permanent cryptographic identity (`.swarm_identity`) on its first boot. 

### Local mDNS Testing
To simulate a multi-device BFT mesh locally:
* **Gateway:** `cargo run --bin swarm-node -- gateway --port 3000`
* **Worker 1:** `cd worker1_dir && cargo run --manifest-path ../Cargo.toml --bin swarm-node -- start --shard 1`
* **Worker 2:** `cd worker2_dir && cargo run --manifest-path ../Cargo.toml --bin swarm-node -- start --shard 2`
*(Note: Because of the unique identity generation, Workers must be run in separate directories during local testing to avoid PeerId collisions).*

### Global Mesh Deployment (WAN)
To connect a mobile Edge Worker to a Public Cloud Gateway:
1. Deploy the Gateway to a public VPS (e.g., Oracle Cloud) and open TCP ports 3000 (API) and 4000 (Mesh) via both the Cloud Provider Firewall and the OS `iptables`.
2. Hardcode the Gateway's explicit Libp2p Multiaddress into the Worker's dial command:
   `let cloud_gateway: libp2p::Multiaddr = "/ip4/<PUBLIC_IP>/tcp/4000/p2p/<GATEWAY_PEER_ID>".parse().unwrap();`
3. Launch the Worker over a cellular or external Wi-Fi network.

## 2. Deploying Stateful Smart Contracts
The REST API accepts JSON payloads for decentralized execution.
* **Endpoint:** `POST /api/v1/jobs`
* **Payload Format:** `{"shards": 1, "wasm_base64": "<BASE64_STRING>", "dataset": ["item1", "item2"]}`

## 3. Checking Status & Consensus
Query the Gateway's Axum API to view distributed execution status and the resulting Output Hash.
* **Endpoint:** `GET /api/v1/jobs/:id`

## 4. Fetching Data from the Mesh
Once a job achieves `Status: COMPLETED`, you can download the resulting output file directly from the decentralized Worker's Virtual File System using its Consensus Hash.
* **Endpoint:** `GET /api/v1/data/:hash`
