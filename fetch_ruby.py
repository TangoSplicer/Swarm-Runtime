import urllib.request

print("Downloading Ruby WASI binary...")
url = "https://cdn.jsdelivr.net/npm/ruby-3_2-wasm-wasi@2.1.0/dist/ruby.wasm"
req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})

try:
    with urllib.request.urlopen(req) as response:
        with open("ruby.wasm", "wb") as out:
            out.write(response.read())
        print("✅ ruby.wasm downloaded successfully!")
except Exception as e:
    print(f"❌ Error downloading: {e}")
