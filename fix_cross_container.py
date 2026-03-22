import re

with open('.github/workflows/ci.yml', 'r') as f:
    yaml = f.read()

# 1. If the workflow uses the actions-rs/cargo plugin, toggle cross dynamically
yaml = re.sub(
    r'use-cross:\s*true', 
    "use-cross: ${{ matrix.target == 'aarch64-linux-android' }}", 
    yaml
)

# 2. If the workflow executes raw bash commands, inject a dynamic shell toggle
def replace_raw_cross(match):
    indent = match.group(1)
    cmd = match.group(2)
    return f"{indent}run: |\n{indent}  if [ \"${{{{ matrix.target }}}}\" == \"x86_64-unknown-linux-gnu\" ]; then\n{indent}    cargo {cmd}\n{indent}  else\n{indent}    cross {cmd}\n{indent}  fi"

yaml = re.sub(r'(^[ \t]+)run:\s*cross\s+(.*)$', replace_raw_cross, yaml, flags=re.MULTILINE)

with open('.github/workflows/ci.yml', 'w') as f:
    f.write(yaml)

print("✅ Matrix payload patched to bypass the broken Linux Docker container!")
