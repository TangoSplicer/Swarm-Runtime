mod gateway;
mod worker;
mod types;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use ed25519_dalek::SigningKey;

#[derive(Parser)]
#[command(name = "swarm-node")]
#[command(about = "Swarm Runtime - Unified Node & CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an Edge Worker node
    Start {
        #[arg(long)]
        shard: u64,
    },
    /// Start the Orchestration Gateway
    Gateway {
        #[arg(long, default_value = "3000")]
        port: u16,
    },
    /// Fetch a file from the network using its SHA-256 Hash
    Fetch {
        hash: String,
        #[arg(long, default_value = "http://127.0.0.1:3000")]
        gateways: String,
    },
    /// Deploy a script to the Swarm network
    Deploy {
        file: String,
        #[arg(short, long, default_value = "python")]
        lang: String,
        #[arg(long, default_value = "http://127.0.0.1:3000")]
        gateways: String,
    },
    /// Check the status and output of a deployed job
    Status {
        job_id: String,
        #[arg(long, default_value = "http://127.0.0.1:3000")]
        gateways: String,
    }
}

#[derive(Serialize)]
struct DeployMetadata {
    dataset: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct JobStatusResponse {
    status: String,
    total_sum: i32,
    _breakdown: Vec<(u32, i32)>,
    hashes: Vec<(u32, String)>,
    missing_shards: Vec<u32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // 1. Unified Cryptographic Identity Loading
    let id_path = ".swarm_identity";
    let signing_key = if let Ok(bytes) = fs::read(id_path) {
        let mut key_bytes = [0u8; 32];
        let len = std::cmp::min(bytes.len(), 32);
        key_bytes[..len].copy_from_slice(&bytes[..len]);
        if cli.is_node_command() {
            println!("🔑 Loaded existing cryptographic identity from .swarm_identity");
        }
        SigningKey::from_bytes(&key_bytes)
    } else {
        if cli.is_node_command() {
            println!("🌱 Generating new cryptographic identity...");
        }
        let mut key_bytes = [0u8; 32];
        let mut file = std::fs::File::open("/dev/urandom").expect("Failed to open /dev/urandom");
        use std::io::Read;
        file.read_exact(&mut key_bytes).expect("Failed to read random bytes");
        let key = SigningKey::from_bytes(&key_bytes);
        fs::write(id_path, key.to_bytes()).context("Failed to write identity")?;
        key
    };

    let verifying_key = signing_key.verifying_key();
    let seed = signing_key.to_bytes();

    // 2. Command Router
    match &cli.command {
        Commands::Start { shard } => {
            worker::run_worker(*shard, verifying_key, seed).await?;
        },
        Commands::Gateway { port } => {
            gateway::run_gateway(*port, signing_key).await?;
        },
        Commands::Deploy { file, lang, gateways } => {
            let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();
            println!("🚀 Preparing deployment for: {}", file);

            let (wasm_bytes, dataset) = match lang.to_lowercase().as_str() {
                "wasm" => {
                    let wasm_bytes = fs::read(file).context("Failed to read .wasm file")?;
                    (wasm_bytes, vec!["EXECUTE_NATIVE_WASM".to_string()])
                },
                "python" | "js" | "javascript" | "lua" | "ruby" | "rb" | "php" | "sqlite" | "sql" => {
                    let code = fs::read_to_string(file).with_context(|| format!("Failed to read file: {}", file))?;
                    let identifier = match lang.to_lowercase().as_str() {
                        "python" => "POLYGLOT:PYTHON",
                        "js" | "javascript" => "POLYGLOT:JS",
                        "lua" => "POLYGLOT:LUA",
                        "ruby" | "rb" => "POLYGLOT:RUBY",
                        "php" => "POLYGLOT:PHP",
                        "sqlite" | "sql" => "POLYGLOT:SQLITE",
                        _ => unreachable!(),
                    };
                    (identifier.as_bytes().to_vec(), vec![code])
                },
                _ => anyhow::bail!("Unsupported language: {}. Supported: python, js, lua, ruby, php, sqlite, wasm", lang),
            };

            let metadata = DeployMetadata { dataset };
            let metadata_json = serde_json::to_string(&metadata)?;

            let gw_list: Vec<&str> = gateways.split(',').map(|s| s.trim()).collect();
            let mut success = false;

            // HA Routing Loop: Try gateways until one succeeds
            for gw in gw_list {
                let wasm_part = reqwest::multipart::Part::bytes(wasm_bytes.clone())
                    .file_name(file.clone())
                    .mime_str("application/octet-stream")?;
                let metadata_part = reqwest::multipart::Part::text(metadata_json.clone())
                    .mime_str("application/json")?;

                let form = reqwest::multipart::Form::new().part("wasm", wasm_part).part("metadata", metadata_part);
                let url = format!("{}/api/v1/jobs", gw.trim_end_matches('/'));
                
                println!("📡 Dispatching payload to Gateway at {}...", url);
                
                if let Ok(res) = client.post(&url).multipart(form).send().await {
                    if res.status().is_success() {
                        println!("✅ Deployment Successful via {}!\n   Gateway Response: {}", gw, res.text().await.unwrap_or_default());
                        success = true;
                        break;
                    } else {
                        println!("⚠️ Failed on {} (Status: {})\n   Error: {}", gw, res.status(), res.text().await.unwrap_or_default());
                    }
                } else {
                    println!("⚠️ Failed to connect to {}", gw);
                }
            }
            
            if !success {
                println!("❌ Deployment Failed across all federated Gateways.");
            }
        },
        Commands::Status { job_id, gateways } => {
            let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();
            let gw_list: Vec<&str> = gateways.split(',').map(|s| s.trim()).collect();
            
            for gw in gw_list {
                let url = format!("{}/api/v1/jobs/{}", gw.trim_end_matches('/'), job_id);
                if let Ok(res) = client.get(&url).send().await {
                    if res.status().is_success() {
                        let status_data: JobStatusResponse = res.json().await?;
                        println!("\n=== 📊 Swarm Job Status ===");
                        println!("Status:          {}", status_data.status.to_uppercase());
                        if status_data.status == "completed" {
                            println!("Consensus Hash:  {}", status_data.hashes.first().map(|(_, h)| h.as_str()).unwrap_or("NONE"));
                            println!("Numeric Result:  {}", status_data.total_sum);
                        } else {
                            println!("Missing Shards:  {:?}", status_data.missing_shards);
                        }
                        println!("===========================\n");
                        return Ok(());
                    }
                }
            }
            println!("❌ Failed to retrieve status from any federated Gateway.");
        },
        Commands::Fetch { hash, gateways } => {
            let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(15)).build().unwrap();
            let gw_list: Vec<&str> = gateways.split(',').map(|s| s.trim()).collect();
            
            for gw in gw_list {
                let url = format!("{}/api/v1/data/{}", gw.trim_end_matches('/'), hash);
                if let Ok(response) = client.get(&url).send().await {
                    if let Ok(bytes) = response.bytes().await {
                        let filename = format!("download_{}.bin", &hash[..8]);
                        let _ = fs::write(&filename, &bytes);
                        println!("✅ Success! File downloaded via {} to: {}", gw, filename);
                        return Ok(());
                    }
                }
            }
            println!("❌ Failed to fetch file from any federated Gateway.");
        }
    }
    Ok(())
}

impl Cli {
    fn is_node_command(&self) -> bool {
        matches!(self.command, Commands::Start { .. } | Commands::Gateway { .. })
    }
}
