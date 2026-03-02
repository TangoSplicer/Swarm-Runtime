import urllib.request

print("Downloading official PHP WASI binary from Unpkg CDN...")
url = "https://unpkg.com/@antonz/php-wasi/dist/php-cgi.wasm"
req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})

try:
    with urllib.request.urlopen(req) as response:
        with open("php.wasm", "wb") as out:
            out.write(response.read())
        print("✅ php.wasm downloaded successfully!")
except Exception as e:
    print(f"❌ Error downloading: {e}")
