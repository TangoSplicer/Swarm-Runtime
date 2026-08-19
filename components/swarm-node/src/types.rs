#![allow(clippy::type_complexity)]
use dashmap::DashMap;
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;
use synapse::SwarmRequest;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeKind {
    Wasm,
    Python,
    JavaScript,
    Lua,
    Ruby,
    Php,
    Sqlite,
}

impl RuntimeKind {
    pub fn polyglot_id(self) -> &'static str {
        match self {
            Self::Wasm => "POLYGLOT:WASM",
            Self::Python => "POLYGLOT:PYTHON",
            Self::JavaScript => "POLYGLOT:JS",
            Self::Lua => "POLYGLOT:LUA",
            Self::Ruby => "POLYGLOT:RUBY",
            Self::Php => "POLYGLOT:PHP",
            Self::Sqlite => "POLYGLOT:SQLITE",
        }
    }

    pub fn runtime_file(self) -> Option<&'static str> {
        match self {
            Self::Wasm => None,
            Self::Python => Some("python.wasm"),
            Self::JavaScript => Some("qjs.wasm"),
            Self::Lua => Some("lua.wasm"),
            Self::Ruby => Some("ruby.wasm"),
            Self::Php => Some("php.wasm"),
            Self::Sqlite => Some("sqlite.wasm"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShardedDeployRequest {
    pub dataset: Vec<String>,
    pub runtime: RuntimeKind,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Shard {
    pub parent_task_id: Uuid,
    pub shard_index: u32,
    pub total_shards: u32,
    pub data: Vec<String>,
    pub wasm_image: Vec<u8>,
    pub runtime: RuntimeKind,
    pub target_peer: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShardResult {
    pub job_id: Uuid,
    pub shard_index: u32,
    pub result: i32,
    pub result_hash: String,
    pub state_delta: BTreeMap<String, String>,
    pub execution_timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SignedShardResult {
    pub result: ShardResult,
    pub worker_public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

pub fn shard_result_message(result: &ShardResult) -> serde_json::Result<Vec<u8>> {
    serde_json::to_vec(result)
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
    #[serde(skip)]
    pub peers: HashSet<libp2p::PeerId>,
}

pub struct JobState {
    pub expected_shards: usize,
    pub redundancy: usize,
    pub raw_results:
        HashMap<u32, HashMap<libp2p::PeerId, (i32, String, BTreeMap<String, String>, u64)>>,
    pub verified_results: HashMap<u32, (i32, String, BTreeMap<String, String>, u64)>,
    pub master_state_hash: Option<String>,
    pub created_at: Instant,
    pub assignments: HashMap<u32, HashMap<libp2p::PeerId, Instant>>,
    pub shards_data: HashMap<u32, Shard>,
    pub unassigned_dataset: Option<Vec<String>>,
    pub wasm_image: Vec<u8>,
    pub runtime: RuntimeKind,
}

pub enum NodeCommand {
    Unicast(libp2p::PeerId, SwarmRequest),
    GatewaySync(String),
    Disconnect(libp2p::PeerId),
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
    pub admission_key: VerifyingKey,
    pub used_admission_nonces: Arc<DashMap<Uuid, Instant>>,
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
    pub master_state_hash: Option<String>,
}
