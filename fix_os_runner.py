import re
import os

with open('.github/workflows/ci.yml', 'r') as f:
    yaml = f.read()

# Strip out the GITHUB_ENV Clang injection step entirely
yaml = re.sub(
    r'\s*- name: Force Clang for Cryptography\s*run: \|\s*echo "CC=clang" >> \$GITHUB_ENV\s*echo "CXX=clang\+\+" >> \$GITHUB_ENV', 
    '', 
    yaml
)

# Upgrade the runner OS to bypass the GCC 11 bug completely
yaml = yaml.replace('runs-on: ubuntu-latest', 'runs-on: ubuntu-24.04')
yaml = yaml.replace('runs-on: ubuntu-22.04', 'runs-on: ubuntu-24.04')

with open('.github/workflows/ci.yml', 'w') as f:
    f.write(yaml)

print("✅ Workflow upgraded to Ubuntu 24.04 and Clang hacks removed!")
