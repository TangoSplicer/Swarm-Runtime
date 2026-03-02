import urllib.request, tarfile, io

print("Downloading official Ruby WASIp1 Command Module from GitHub...")
url = "https://github.com/ruby/ruby.wasm/releases/latest/download/ruby-3.3-wasm32-unknown-wasip1-full.tar.gz"
req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})

try:
    with urllib.request.urlopen(req) as response:
        with tarfile.open(fileobj=io.BytesIO(response.read()), mode="r:gz") as tar:
            for member in tar.getmembers():
                # Extract specifically the command-line executable
                if member.name.endswith("usr/local/bin/ruby"):
                    f = tar.extractfile(member)
                    with open("ruby.wasm", "wb") as out:
                        out.write(f.read())
                    print("✅ CLI ruby.wasm downloaded and extracted successfully!")
                    break
except Exception as e:
    print(f"❌ Error downloading: {e}")
