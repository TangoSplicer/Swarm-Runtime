import re

path = "components/synapse/src/lib.rs"
with open(path, "r") as f:
    code = f.read()

# Strip out the hallucinated block and replace it with the correct 0.51 builder syntax
bad_block_pattern = r'\{\s*let mut cfg = request_response::Config::default\(\);\s*cfg\.set_max_request_size\(.*?\);\s*cfg\.set_max_response_size\(.*?\);\s*cfg\.set_request_timeout\(.*?\);\s*cfg\s*\}'
correct_syntax = "request_response::Config::default().with_request_timeout(std::time::Duration::from_secs(300))"

new_code = re.sub(bad_block_pattern, correct_syntax, code)

with open(path, "w") as f:
    f.write(new_code)
    
print("✅ Synapse P2P code repaired. Timeout extended to 300 seconds.")
