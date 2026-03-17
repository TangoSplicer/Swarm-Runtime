# Swarm Runtime: Developer User Manual (v0.26.0)

## 1. Starting the Mesh & Cryptographic Identity
Every node dynamically generates a permanent cryptographic identity (`.swarm_identity`) on its first boot from `/dev/urandom`.

### Global Mesh Deployment (WAN)
To connect a mobile Edge Worker to a Public Cloud Gateway:
1. Deploy the Gateway to a public VPS (e.g., Oracle Cloud).
2. Hardcode the Gateway's explicit Libp2p Multiaddress into the Worker's dial command:
   `/ip4/<PUBLIC_IP>/tcp/4000/p2p/<GATEWAY_PEER_ID>`
3. Launch the Worker: `cargo run --release --bin swarm-node -- start --shard 1`

## 2. Deploying Stateful Smart Contracts
Use the built-in CLI to stream Polyglot scripts or raw WebAssembly binaries to the Gateway.

* **Deploy Python (Federated):** `cargo run --bin swarm-node -- deploy test_payloads/test_python.py --lang python --gateways http://<PRIMARY_IP>:3000,http://<SECONDARY_IP>:3000`
* **Deploy JS:** `cargo run --bin swarm-node -- deploy test_payloads/test_js.js --lang js --gateway http://<PUBLIC_IP>:3000`
* **Deploy WASM:** `cargo run --bin swarm-node -- deploy runtimes/calculator.wasm --lang wasm --gateway http://<PUBLIC_IP>:3000`

## 3. Checking Status & Consensus
Query the Gateway to view distributed execution status and the resulting Output Hash.
* **Check Job:** `cargo run --bin swarm-node -- status <JOB_ID> --gateway http://<PUBLIC_IP>:3000`

## 4. Fetching Data from the Mesh
Once a job completes, any DHT-pinned output files can be retrieved via their SHA-256 hash.
* **Fetch File:** `cargo run --bin swarm-node -- fetch <HASH> --gateway http://<PUBLIC_IP>:3000`
