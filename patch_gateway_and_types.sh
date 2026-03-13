#!/bin/bash

# ==========================================
# 1. UPDATE DATA STRUCTURES (types.rs)
# ==========================================
TYPES_FILE="components/swarm-node/src/types.rs"

# Remove the base64 field from the JSON payload requirement
sed -i '/pub wasm_base64: String,/d' "$TYPES_FILE"

# Convert all wasm_image representations to raw bytes (affects Shard and JobState)
sed -i 's/pub wasm_image: String,/pub wasm_image: Vec<u8>,/g' "$TYPES_FILE"

# ==========================================
# 2. UPDATE MULTIPART HANDLER (gateway.rs)
# ==========================================
GATEWAY_FILE="components/swarm-node/src/gateway.rs"

# Swap Axum extractor from Json to Multipart
sed -i 's/Json(payload): Json<ShardedDeployRequest>/mut multipart: axum::extract::Multipart/g' "$GATEWAY_FILE"

# Inject the async multipart streaming logic immediately after task_id generation
sed -i '/let task_id = Uuid::new_v4();/a \
    let mut wasm_bytes = Vec::new();\
    let mut dataset = Vec::new();\
\
    while let Ok(Some(field)) = multipart.next_field().await {\
        let name = field.name().unwrap_or("").to_string();\
        if name == "wasm" {\
            if let Ok(data) = field.bytes().await {\
                wasm_bytes = data.to_vec();\
            }\
        } else if name == "metadata" {\
            if let Ok(text) = field.text().await {\
                if let Ok(payload) = serde_json::from_str::<ShardedDeployRequest>(&text) {\
                    dataset = payload.dataset;\
                }\
            }\
        }\
    }' "$GATEWAY_FILE"

# Update instantiation variables to use the newly parsed multipart variables
sed -i 's/unassigned_dataset: Some(payload.dataset.clone()),/unassigned_dataset: Some(dataset.clone()),/g' "$GATEWAY_FILE"
sed -i 's/wasm_image: payload.wasm_base64,/wasm_image: wasm_bytes,/g' "$GATEWAY_FILE"
sed -i 's/payload.dataset.len()/dataset.len()/g' "$GATEWAY_FILE"

# ==========================================
# 3. FIX HASHING & LOGIC FOR Vec<u8> (gateway.rs)
# ==========================================
# Fix emptiness checks to compare against byte literals
sed -i 's/job.wasm_image != "NONE"/job.wasm_image != b"NONE"/g' "$GATEWAY_FILE"
sed -i 's/guard.wasm_image != "NONE"/guard.wasm_image != b"NONE"/g' "$GATEWAY_FILE"

# Fix Sha256 hashing to reference the vector natively instead of .as_bytes()
sed -i 's/hasher.update(job.wasm_image.as_bytes());/hasher.update(\&job.wasm_image);/g' "$GATEWAY_FILE"
sed -i 's/hasher.update(guard.wasm_image.as_bytes());/hasher.update(\&guard.wasm_image);/g' "$GATEWAY_FILE"

# Safely extend the raw bytes to attach the Latest State Hash without format!
sed -i 's/out_wasm_image = format!("{}|STATE:{}", out_wasm_image, latest_state.value());/let mut final_wasm = out_wasm_image.clone(); final_wasm.extend_from_slice(b"|STATE:"); final_wasm.extend_from_slice(latest_state.value().as_bytes()); out_wasm_image = final_wasm;/g' "$GATEWAY_FILE"

echo "✅ Surgical patches applied successfully."
