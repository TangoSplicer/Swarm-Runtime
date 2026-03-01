import os
import shutil
import zipfile

print("1. Restructuring CPython Standard Library...")
if not os.path.exists("lib/python3.11"):
    if os.path.exists("python-wasi.zip"):
        with zipfile.ZipFile("python-wasi.zip", 'r') as z:
            z.extractall("cpython_temp")
        
        # Deep search for the python3.11 folder
        for root, dirs, files in os.walk("cpython_temp"):
            if "python3.11" in dirs:
                target = os.path.join(root, "python3.11")
                os.makedirs("lib", exist_ok=True)
                shutil.move(target, "lib/python3.11")
                print("✅ Standard library successfully locked into ./lib/python3.11")
                break
else:
    print("✅ Standard library already exists at ./lib/python3.11")
