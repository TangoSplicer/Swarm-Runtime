use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct ShardedDeployRequest {
    pub wasm_base64: String,
    pub dataset: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Shard {
    pub parent_task_id: Uuid,
    pub shard_index: u32,
    pub total_shards: u32,
    pub data: Vec<i32>,
    pub wasm_image: String,
    pub target_peer: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShardResult {
    pub job_id: Uuid,
    pub shard_index: u32,
    pub result: i32,
}
