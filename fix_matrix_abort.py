with open('.github/workflows/ci.yml', 'r') as f:
    yaml = f.read()

# Inject fail-fast: false safely into the strategy block
if 'fail-fast: false' not in yaml:
    yaml = yaml.replace('strategy:\n      matrix:', 'strategy:\n      fail-fast: false\n      matrix:')

with open('.github/workflows/ci.yml', 'w') as f:
    f.write(yaml)

print("✅ Matrix fail-fast disabled!")
