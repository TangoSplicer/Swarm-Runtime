import re

with open('.github/workflows/ci.yml', 'r') as f:
    yaml = f.read()

# Strip any partial previous attempts to avoid duplication
yaml = yaml.replace("CC=clang CXX=clang++ ", "")

# Inject the compiler override directly into every cargo bash execution
yaml = re.sub(r'(\s+)cargo (build|test|check|clippy|fmt)', r'\1CC=clang CXX=clang++ cargo \2', yaml)

with open('.github/workflows/ci.yml', 'w') as f:
    f.write(yaml)

print("✅ Compiler overrides hardcoded directly into workflow execution!")
