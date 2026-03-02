📘 Swarm Runtime: Developer User Manual (v0.20.3)
Welcome to Swarm Runtime. This guide outlines how to initialize the network and deploy your first polyglot script across the decentralized mesh.
1. Starting the Mesh
Swarm requires at least one Gateway (to schedule jobs) and two Workers (to execute tasks and establish redundancy).
Open three separate Termux sessions and run:
Terminal 1 (Gateway): cargo run --bin swarm-node -- gateway --port 3000
Terminal 2 (Worker 1): cargo run --bin swarm-node -- start --shard 1
Terminal 3 (Worker 2): cargo run --bin swarm-node -- start --shard 2
Wait until the Gateway logs 📡 CONNECTED for both workers. The workers will continuously broadcast hardware telemetry to the Gateway via Gossipsub.
2. Writing a Polyglot Script
Create a Python file on your local machine. The script will be executed inside a highly secure WebAssembly sandbox. It can write outputs to the /data virtual directory.
Create app.py:

import sys
output_path = "/data/output.txt"
print(f"Executing decentralized Python! Writing to {output_path}")
with open(output_path, "w") as f:
    f.write("Hello from the Swarm!")

3. Deploying with the CLI
Use the swarm-cli tool to deploy your script to the network. The CLI automatically packages your code and targets the Gateway's REST API.

cargo run --bin swarm-cli -- deploy app.py --lang python

Expected Output:

🚀 Preparing deployment for: app.py
📡 Dispatching payload to Gateway at http://127.0.0.1:3000/api/v1/jobs...
✅ Deployment Successful!
   Gateway Response: {"job_id":"<YOUR_JOB_ID>","status":"pending_scheduler"}

4. Checking Execution Status
Behind the scenes, the Gateway unicasts your job to both workers. The workers execute the Python code, hash the output files using SHA-256, and announce the files to the Kademlia DHT. The Gateway waits for the hashes to match to establish Byzantine Fault Tolerance.
Check the cryptographic consensus of your job using the ID provided during deployment:

cargo run --bin swarm-cli -- status <YOUR_JOB_ID>

Expected Output:

=== 📊 Swarm Job Status ===
Status:          COMPLETED
Consensus Hash:  e3b0c442...
Numeric Result:  0
Verified Shards: 2
===========================

