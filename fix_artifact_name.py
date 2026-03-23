with open('.github/workflows/ci.yml', 'r') as f:
    content = f.read()

# 1. Make the artifact name strictly unique per matrix job
content = content.replace(
    "name: swarm-android-binaries", 
    "name: swarm-binaries-${{ matrix.target }}"
)

# 2. Fix the display name of the step so it makes sense for both OS types
content = content.replace(
    "name: Upload Android Binaries", 
    "name: Upload Compiled Binaries"
)

with open('.github/workflows/ci.yml', 'w') as f:
    f.write(content)

print("✅ Artifact name conflict resolved!")
