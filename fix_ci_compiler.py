with open('.github/workflows/ci.yml', 'r') as f:
    yaml = f.read()

# Inject Clang as the default C/C++ compiler globally for the runner
if 'CC: clang' not in yaml:
    yaml = yaml.replace('CARGO_TERM_COLOR: always', 'CARGO_TERM_COLOR: always\n  CC: clang\n  CXX: clang++')

with open('.github/workflows/ci.yml', 'w') as f:
    f.write(yaml)

print("✅ CI Pipeline configured to use Clang for secure crypto compilation!")
