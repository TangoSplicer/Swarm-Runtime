import urllib.request

print("Downloading Lua WASI binary...")
# This is a highly stable WASI-compiled Lua 5.4 binary hosted on Unpkg/GitHub
url = "https://unpkg.com/@antonz/lua-wasi/dist/lua.wasm"
req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})

try:
    with urllib.request.urlopen(req) as response:
        with open("lua.wasm", "wb") as out:
            out.write(response.read())
        print("✅ lua.wasm downloaded successfully!")
except Exception as e:
    print(f"❌ Error downloading: {e}")
