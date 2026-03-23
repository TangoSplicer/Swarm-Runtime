with open('.github/workflows/ci.yml', 'r') as f:
    content = f.read()

# 1. Remove the Android-only restriction
content = content.replace("        if: matrix.target == 'aarch64-linux-android'\n", "")

# 2. Swap the hardcoded Android paths for dynamic matrix variables
content = content.replace("target/aarch64-linux-android/release", "target/${{ matrix.target }}/release")

with open('.github/workflows/ci.yml', 'w') as f:
    f.write(content)

print("✅ Pipeline dynamically updated to save both Linux and Android binaries!")
