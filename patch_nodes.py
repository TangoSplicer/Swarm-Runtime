import re

def remove_mdns_arms(text):
    while "SynapseBehaviorEvent::Mdns" in text:
        start_idx = text.find("SwarmEvent::Behaviour(SynapseBehaviorEvent::Mdns")
        if start_idx == -1: break
        
        brace_start = text.find("{", start_idx)
        count = 1
        curr = brace_start + 1
        while count > 0 and curr < len(text):
            if text[curr] == '{': count += 1
            elif text[curr] == '}': count -= 1
            curr += 1
            
        while curr < len(text) and text[curr] in [' ', '\n', '\t']:
            curr += 1
        if curr < len(text) and text[curr] == ',':
            curr += 1
            
        text = text[:start_idx] + text[curr:]
    return text

# --- 1. Patch the Gateway ---
with open('components/swarm-node/src/gateway.rs', 'r') as f:
    gw = f.read()

gw = remove_mdns_arms(gw)
gw = gw.replace('p2p_node.send_response(', 'let _ = p2p_node.swarm.behaviour_mut().req_res.send_response(')

with open('components/swarm-node/src/gateway.rs', 'w') as f:
    f.write(gw)

# --- 2. Patch the Worker ---
with open('components/swarm-node/src/worker.rs', 'r') as f:
    wk = f.read()

wk = remove_mdns_arms(wk)
wk = wk.replace('p2p_node.send_response(', 'let _ = p2p_node.swarm.behaviour_mut().req_res.send_response(')

# Strip unused imports
wk = re.sub(r'use libp2p::\{[^}]*mdns[^}]*\};', 'use libp2p::{request_response, swarm::SwarmEvent};', wk)
wk = re.sub(r'use std::collections::\{[^}]*HashMap[^}]*\};', 'use std::collections::BTreeMap;', wk)
wk = re.sub(r'use std::time::\{[^}]*Duration[^}]*\};', 'use std::time::{SystemTime, UNIX_EPOCH};', wk)
wk = wk.replace('use sysinfo::System;\n', '')
wk = wk.replace('use tokio::sync::Mutex;\n', '')

# Fix Dalek Cryptography Signature
wk = re.sub(
    r'if let Ok\(signature\) = Signature::from_bytes\(signed_payload\.signature\.as_slice\(\)\.try_into\(\)\.unwrap_or\(\[0; 64\]\)\)',
    r'let signature = Signature::from_bytes(signed_payload.signature.as_slice().try_into().unwrap_or(&[0u8; 64]));\n                                        if true',
    wk
)

# Fix Judge Execution API
old_judge = r'let mut judge = Judge::new\(&shard_data\.wasm_image, &shard_data\.data\);\s*let \(execution_result_code, execution_result_hash\) = judge\.execute\(\)\.await\.unwrap_or\(\(-1, "ERROR"\.to_string\(\)\)\);'
new_judge = r'''let mut hasher = sha2::Sha256::new();
                                                hasher.update(&shard_data.wasm_image);
                                                let contract_id = hex::encode(hasher.finalize());
                                                let state_path = safe_state_path(&contract_id).unwrap_or_else(|| "./rootfs/data/default.state".to_string());

                                                let mut judge = Judge::new(None).unwrap();
                                                let (execution_result_code, execution_result_hash, _) = judge.execute(
                                                    &shard_data.wasm_image,
                                                    &shard_data.data,
                                                    "POLYGLOT:WASM",
                                                    &state_path
                                                ).unwrap_or((-1, "ERROR".to_string(), None));'''
wk = re.sub(old_judge, new_judge, wk)

# Remove the old redundant hasher block further down the file
redundant_hasher = r'// Determine contract ID for state path\s*let mut hasher = Sha256::new\(\);\s*hasher\.update\(&shard_data\.wasm_image\);\s*let contract_id = hex::encode\(hasher\.finalize\(\)\);\s*let state_path = safe_state_path\(&contract_id\)\s*\.unwrap_or_else\(\|\| "\./rootfs/data/default\.state"\.to_string\(\)\);'
wk = re.sub(redundant_hasher, '', wk)

with open('components/swarm-node/src/worker.rs', 'w') as f:
    f.write(wk)

print("✅ AST Patching Complete!")
