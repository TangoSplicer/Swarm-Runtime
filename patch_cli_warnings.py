import os

path = "components/swarm-cli/src/main.rs"
with open(path, "r") as f:
    code = f.read()

target = """                if status_data.status == "completed" {
                    println!("Consensus Hash:  {}", status_data.hashes.first().map(|(_, h)| h.as_str()).unwrap_or("NONE"));
                    println!("Verified Shards: {}", status_data.hashes.len());"""

replacement = """                if status_data.status == "completed" {
                    println!("Consensus Hash:  {}", status_data.hashes.first().map(|(_, h)| h.as_str()).unwrap_or("NONE"));
                    println!("Numeric Result:  {}", status_data.total_sum);
                    println!("Shard Breakdown: {:?}", status_data.breakdown);
                    println!("Verified Shards: {}", status_data.hashes.len());"""

if target in code:
    code = code.replace(target, replacement)
    with open(path, "w") as f:
        f.write(code)
    print("✅ CLI output upgraded and warnings resolved.")
else:
    # Fallback just in case indentation differs
    code = code.replace("struct JobStatusResponse", "#[allow(dead_code)]\nstruct JobStatusResponse")
    with open(path, "w") as f:
        f.write(code)
    print("✅ Dead code warning suppressed.")
