use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct ShardedDeployRequest {
    pub wasm_base64: String,
    pub dataset: Vec<String>, // UPGRADED: Now accepts complex strings/JSON
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Shard {
    pub parent_task_id: Uuid,
    pub shard_index: u32,
    pub total_shards: u32,
    pub data: Vec<String>,    // UPGRADED: Shards contain string chunks
    pub wasm_image: String,
    pub target_peer: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShardResult {
    pub job_id: Uuid,
    pub shard_index: u32,
    pub result: i32,
    pub result_hash: String,  // NEW: SHA-256 Hash of the execution state
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SignedPayload {
    pub payload_json: String, 
    pub expires_at: u64,      
    pub signature: Vec<u8>,   
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Telemetry {
    pub peer_id: String,
    pub cpu_usage: f32,    
    pub free_ram_mb: u64,  
}
