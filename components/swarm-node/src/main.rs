#![allow(clippy::never_loop)]
mod gateway;
mod types;
mod worker;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ed25519_dalek::SigningKey;
use lazarus::CriticalFailure;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::signal;
use tokio::sync::mpsc; // PHASE 15: Imported Lazarus Fault Tolerance

#[derive(Parser)]
#[command(name = "swarm-node")]
#[command(version = env!("CARGO_PKG_VERSION"))]
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
        #[arg(long)]
        bootnode: String,
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
    },
}

#[derive(Serialize)]
struct DeployMetadata {
    dataset: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct JobStatusResponse {
    status: String,
    #[serde(default)]
    total_sum: i32,
    #[serde(rename = "breakdown", default)]
    _breakdown: Vec<(u32, i32)>,
    #[serde(default)]
    hashes: Vec<(u32, String)>,
    #[serde(default)]
    missing_shards: Vec<u32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.is_node_command() {
        println!(
            "🐝 Swarm Runtime v{} - Initializing...",
            env!("CARGO_PKG_VERSION")
        );
    }

    // Unified Cryptographic Identity Loading
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
        let mut csprng = OsRng;
        let key = SigningKey::generate(&mut csprng);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(id_path)
                .context("Failed to open identity file with strict permissions")?;
            std::io::Write::write_all(&mut file, &key.to_bytes()).context("Failed to write identity")?;
        }
        #[cfg(not(unix))]
        {
            std::fs::write(id_path, key.to_bytes()).context("Failed to write identity")?;
        }
        key
    };

    let verifying_key = signing_key.verifying_key();
    let seed = signing_key.to_bytes();

    match &cli.command {
        Commands::Start { shard, bootnode } => {
            // PHASE 15: Lazarus Monitoring for the Edge Worker
            let (alert_tx, mut alert_rx) = mpsc::channel::<CriticalFailure>(32);
            let alert_tx_clone = alert_tx.clone();

            let worker_shard = *shard;
            let worker_key = verifying_key;
            let worker_seed = seed;

            let worker_bootnode = bootnode.clone();
            tokio::spawn(async move {
                if let Err(e) =
                    worker::run_worker(worker_shard, worker_key, worker_seed, worker_bootnode).await
                {
                    let _ = alert_tx_clone
                        .send(CriticalFailure {
                            service_name: format!("EdgeWorker-Shard-{}", worker_shard),
                            error_message: e.to_string(),
                        })
                        .await;
                }
            });

            println!("🛡️ Lazarus Fault Tolerance Engine monitoring Worker...");
            loop {
                tokio::select! {
                    Some(failure) = alert_rx.recv() => {
                        eprintln!("\n🔥 FATAL: Swarm Runtime caught a critical failure!");
                        eprintln!("Service: {}", failure.service_name);
                        eprintln!("Error: {}", failure.error_message);
                        eprintln!("Initiating graceful global shutdown...");
                        break;
                    }
                    _ = signal::ctrl_c() => {
                        println!("\n🛑 Received termination signal. Shutting down Worker...");
                        break;
                    }
                }
            }
        }
        Commands::Gateway { port } => {
            // PHASE 15: Lazarus Monitoring for the Orchestration Gateway
            let (alert_tx, mut alert_rx) = mpsc::channel::<CriticalFailure>(32);
            let alert_tx_clone = alert_tx.clone();

            let gw_port = *port;
            let gw_key = signing_key.clone();

            tokio::spawn(async move {
                if let Err(e) = gateway::run_gateway(gw_port, gw_key).await {
                    let _ = alert_tx_clone
                        .send(CriticalFailure {
                            service_name: "OrchestrationGateway".to_string(),
                            error_message: e.to_string(),
                        })
                        .await;
                }
            });

            println!(
                "🛡️ Lazarus Fault Tolerance Engine monitoring Gateway on port {}...",
                port
            );
            loop {
                tokio::select! {
                    Some(failure) = alert_rx.recv() => {
                        eprintln!("\n🔥 FATAL: Swarm Runtime caught a critical failure!");
                        eprintln!("Service: {}", failure.service_name);
                        eprintln!("Error: {}", failure.error_message);
                        eprintln!("Initiating graceful global shutdown...");
                        break;
                    }
                    _ = signal::ctrl_c() => {
                        println!("\n🛑 Received termination signal. Shutting down Gateway...");
                        break;
                    }
                }
            }
        }
        Commands::Deploy {
            file,
            lang,
            gateways,
        } => {
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .unwrap();
            println!("🚀 Preparing deployment for: {}", file);

            let meta = fs::metadata(file).context("Failed to read file metadata")?;
            if meta.len() > 50 * 1024 * 1024 {
                anyhow::bail!("SECURITY ALARM: Payload exceeds 50MB limit. Deployment aborted to prevent OOM.");
            }
            let (wasm_bytes, dataset) = match lang.to_lowercase().as_str() {
                "wasm" => {
                    let wasm_bytes = fs::read(file).context("Failed to read .wasm file")?;
                    if !wasm_bytes.starts_with(b"\0asm") {
                        anyhow::bail!("SECURITY ALARM: Invalid WASM magic number. File is corrupted or malicious.");
                    }
                    (wasm_bytes, vec!["EXECUTE_NATIVE_WASM".to_string()])
                }
                "python" | "js" | "javascript" | "lua" | "ruby" | "rb" | "php" | "sqlite"
                | "sql" => {
                    let code = fs::read_to_string(file)
                        .with_context(|| format!("Failed to read file: {}", file))?;
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
                }
                _ => anyhow::bail!(
                    "Unsupported language: {}. Supported: python, js, lua, ruby, php, sqlite, wasm",
                    lang
                ),
            };

            let metadata = DeployMetadata { dataset };
            let metadata_json = serde_json::to_string(&metadata)?;
            let gw_list: Vec<&str> = gateways.split(',').map(|s| s.trim()).collect();
            let mut success = false;

            for gw in gw_list {
                let wasm_part = reqwest::multipart::Part::bytes(wasm_bytes.clone())
                    .file_name(file.clone())
                    .mime_str("application/wasm")?;

                let metadata_part = reqwest::multipart::Part::text(metadata_json.clone())
                    .mime_str("application/json")?;

                let form = reqwest::multipart::Form::new()
                    .part("wasm", wasm_part)
                    .part("metadata", metadata_part);
                let url = format!("{}/api/v1/jobs", gw.trim_end_matches('/'));

                println!("📡 Dispatching payload to Gateway at {}...", url);

                if let Ok(res) = client.post(&url).multipart(form).send().await {
                    if res.status().is_success() {
                        println!(
                            "✅ Deployment Successful via {}!\n   Gateway Response: {}",
                            gw,
                            res.text().await.unwrap_or_default()
                        );
                        success = true;
                        break;
                    } else {
                        println!(
                            "⚠️ Failed on {} (Status: {})\n   Error: {}",
                            gw,
                            res.status(),
                            res.text().await.unwrap_or_default()
                        );
                    }
                } else {
                    println!("⚠️ Failed to connect to {}", gw);
                }
            }

            if !success {
                println!("❌ Deployment Failed across all federated Gateways.");
            }
        }
        Commands::Status { job_id, gateways } => {
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .unwrap();
            let gw_list: Vec<&str> = gateways.split(',').map(|s| s.trim()).collect();

            for gw in gw_list {
                let url = format!("{}/api/v1/jobs/{}", gw.trim_end_matches('/'), job_id);
                if let Ok(res) = client.get(&url).send().await {
                    if res.status().is_success() {
                        let status_data: JobStatusResponse = res.json().await?;
                        println!("\n=== 📊 Swarm Job Status ===");
                        println!("Status:          {}", status_data.status.to_uppercase());
                        if status_data.status == "completed" {
                            println!(
                                "Consensus Hash:  {}",
                                status_data
                                    .hashes
                                    .first()
                                    .map(|(_, h)| h.as_str())
                                    .unwrap_or("NONE")
                            );
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
        }
        Commands::Fetch { hash, gateways } => {
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .unwrap();
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
        matches!(
            self.command,
            Commands::Start { .. } | Commands::Gateway { .. }
        )
    }
}
