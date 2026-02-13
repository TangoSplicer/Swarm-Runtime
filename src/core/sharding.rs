use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskManifest {
    pub task_id: Uuid,
    pub wasm_hash: String, // To ensure all nodes run the same binary
    pub total_shards: u32,
    pub strategy: ShardingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShardingStrategy {
    /// Splits a numeric range [start, end] into N pieces
    Range { start: u64, end: u64 },
    /// Distributes an array of keys across the mesh
    KeySpace(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shard {
    pub parent_task_id: Uuid,
    pub shard_index: u32,
    pub data_payload: Vec<u8>,
}
