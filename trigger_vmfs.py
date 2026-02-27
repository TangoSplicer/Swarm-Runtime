import urllib.request
import base64
import json
import os

# 1. Create the dummy output file to simulate a Wasm file-write
os.makedirs("./swarm_data", exist_ok=True)
with open("./swarm_data/greetings.txt", "w") as f:
    f.write("The Swarm Virtual File System is ALIVE! Greetings from the decentralized edge.\n")

# 2. Generate a pristine, mathematically correct Wasm binary
# Signature: (param i32 i32) (result i32) -> Returns 42
wasm_bytes = (
    b'\x00\x61\x73\x6d\x01\x00\x00\x00' # Header
    b'\x01\x07\x01\x60\x02\x7f\x7f\x01\x7f' # Type section
    b'\x03\x02\x01\x00' # Function section
    b'\x07\x0b\x01\x07\x65\x78\x65\x63\x75\x74\x65\x00\x00' # Export "execute"
    b'\x0a\x06\x01\x04\x00\x41\x2a\x0b' # Code section
)
b64_payload = base64.b64encode(wasm_bytes).decode('utf-8')

# 3. Fire the request to the Gateway
req = urllib.request.Request("http://localhost:3000/api/v1/jobs", method="POST")
req.add_header("Content-Type", "application/json")
data = json.dumps({
    "wasm_base64": b64_payload, 
    "dataset": ["Triggering the VMFS Sweeper!"]
}).encode('utf-8')

try:
    res = urllib.request.urlopen(req, data=data)
    print("Gateway Response:", res.read().decode('utf-8'))
except Exception as e:
    print("Error:", e)
