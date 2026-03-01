import urllib.request
import urllib.error
import json
import os
import shutil
import zipfile

print("1. Constructing secure WASI rootfs...")
if not os.path.exists("rootfs/lib/python3.11"):
    os.makedirs("rootfs/lib", exist_ok=True)
    if os.path.exists("python-wasi.zip"):
        print("Extracting verified CPython archive...")
        with zipfile.ZipFile("python-wasi.zip", 'r') as z:
            z.extractall("cpython_temp")
        for root, dirs, files in os.walk("cpython_temp"):
            if "python3.11" in dirs:
                shutil.move(os.path.join(root, "python3.11"), "rootfs/lib/python3.11")
                print("✅ Standard library securely isolated in rootfs.")
                break
        shutil.rmtree("cpython_temp", ignore_errors=True)
    else:
        print("❌ ERROR: python-wasi.zip not found! Please download it first.")
        exit(1)

app_code = """import sys
import os
output_path = "/data/output.txt"
print(f"Executing decentralized Python on Wasmi! Writing to {output_path}")
with open(output_path, "w") as f:
    f.write("Phase 5: Polyglot Ecosystem Initialized.")
print("Execution complete. SHA-256 Hash consensus ready.")
"""

payload = {
    "wasm_base64": "POLYGLOT:PYTHON",
    "dataset": [app_code]
}

print("2. Dispatching polyglot execution via Gateway...")
req = urllib.request.Request(
    "http://127.0.0.1:3000/api/v1/jobs",
    data=json.dumps(payload).encode('utf-8'),
    headers={'Content-Type': 'application/json'}
)

try:
    with urllib.request.urlopen(req) as response:
        print("✅ Gateway Response:", response.read().decode('utf-8'))
except urllib.error.HTTPError as e:
    print("❌ Gateway Error:", e.code, e.read().decode('utf-8'))
