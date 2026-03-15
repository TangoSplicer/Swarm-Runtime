use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use ed25519_dalek::SigningKey;
use synapse::SwarmRequest;

#[derive(Serialize, Deserialize, Clone)]
pub struct ShardedDeployRequest {
    pub dataset: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Shard {
    pub parent_task_id: Uuid,
    pub shard_index: u32,
    pub total_shards: u32,
    pub data: Vec<String>,
    pub wasm_image: Vec<u8>,
    pub target_peer: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShardResult {
    pub job_id: Uuid,
    pub shard_index: u32,
    pub result: i32,
    pub result_hash: String,
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

#[derive(Serialize, Clone)]
pub struct SwarmStatus {
    pub version: String,
    pub role: String,
    pub peers_count: usize,
    #[serde(skip)] pub peers: HashSet<libp2p::PeerId>,
}

pub struct JobState {
    pub expected_shards: usize,
    pub redundancy: usize,
    pub raw_results: HashMap<u32, HashMap<libp2p::PeerId, (i32, String)>>,
    pub verified_results: HashMap<u32, (i32, String)>,
    pub created_at: Instant,
    pub assignments: HashMap<u32, HashMap<libp2p::PeerId, Instant>>,
    pub shards_data: HashMap<u32, Shard>,
    pub unassigned_dataset: Option<Vec<String>>,
    pub wasm_image: Vec<u8>,
}

pub enum NodeCommand {
    Unicast(libp2p::PeerId, SwarmRequest),
    Broadcast(String),
    GatewaySync(String),
    Disconnect(libp2p::PeerId),
    PinFile(String),
    FetchFile(String, tokio::sync::oneshot::Sender<Option<Vec<u8>>>),
}

#[allow(dead_code)]
pub struct AppState {
    pub node_tx: tokio::sync::mpsc::Sender<NodeCommand>,
    pub jobs: Arc<DashMap<Uuid, Arc<Mutex<JobState>>>>,
    pub stats: Arc<Mutex<SwarmStatus>>,
    pub health_registry: Arc<DashMap<libp2p::PeerId, u8>>,
    pub pending_dials: Arc<DashMap<libp2p::PeerId, Instant>>,
    pub telemetry_registry: Arc<DashMap<libp2p::PeerId, Telemetry>>,
    pub signing_key: SigningKey,
}

#[derive(Serialize)]
pub struct JobSubmitResponse {
    pub job_id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct JobStatusResponse {
    pub status: String,
    pub total_sum: i32,
    pub breakdown: Vec<(u32, i32)>,
    pub hashes: Vec<(u32, String)>,
    pub missing_shards: Vec<u32>,
}
