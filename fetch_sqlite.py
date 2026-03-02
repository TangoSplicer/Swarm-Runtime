import urllib.request

print("Downloading official SQLite WASI binary...")
url = "https://unpkg.com/@antonz/sqlite-wasi/dist/sqlite.wasm"
req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})

try:
    with urllib.request.urlopen(req) as response:
        with open("sqlite.wasm", "wb") as out:
            out.write(response.read())
        print("✅ sqlite.wasm downloaded successfully!")
except Exception as e:
    print(f"❌ Error downloading: {e}")
