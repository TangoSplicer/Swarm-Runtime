import re

with open('.github/workflows/ci.yml', 'r') as f:
    yaml = f.read()

# 1. Strip out the old, failed regex prefixes
yaml = yaml.replace("CC=clang CXX=clang++ ", "")

# 2. Inject the inescapable $GITHUB_ENV setup right after the checkout step
env_injection = r'\g<0>\n      - name: Force Clang for Cryptography\n        run: |\n          echo "CC=clang" >> $GITHUB_ENV\n          echo "CXX=clang++" >> $GITHUB_ENV'
yaml = re.sub(r'uses:\s*actions/checkout@[^\n]+', env_injection, yaml)

with open('.github/workflows/ci.yml', 'w') as f:
    f.write(yaml)

print("✅ Inescapable GITHUB_ENV override injected!")
