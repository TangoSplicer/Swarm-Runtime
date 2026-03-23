with open('.github/workflows/ci.yml', 'r') as f:
    yaml = f.read()

if 'actions/upload-artifact' not in yaml:
    # Append the upload step using standard GitHub Actions step indentation (6 spaces)
    artifact_block = """
      - name: Upload Android Binaries
        if: matrix.target == 'aarch64-linux-android'
        uses: actions/upload-artifact@v4
        with:
          name: swarm-android-binaries
          path: |
            target/aarch64-linux-android/release/swarm-node
            target/aarch64-linux-android/release/swarm-cli
"""
    with open('.github/workflows/ci.yml', 'a') as f:
        f.write(artifact_block)
    print("✅ Artifact upload step successfully appended!")
else:
    print("⚠️ Artifact step already exists. No changes made.")
