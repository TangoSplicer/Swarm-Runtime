use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct ShardedDeployRequest {
    pub wasm_base64: String,
    pub range_size: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Shard {
    pub parent_task_id: Uuid,
    pub shard_index: u32,
    pub total_shards: u32,
    pub data_range_start: u64,
    pub data_range_end: u64,
    pub wasm_image: String,
}
// ShardAggregation struct removed (handled by AppState in main.rs)
