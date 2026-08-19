#![allow(clippy::never_loop)]
mod gateway;
mod types;
mod worker;

use crate::types::RuntimeKind;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use lazarus::CriticalFailure;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fs;
use synapse::{admission_message, AdmissionToken};
use tokio::signal;
use tokio::sync::mpsc;
use uuid::Uuid; // PHASE 15: Imported Lazarus Fault Tolerance

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
        #[arg(long)]
        trusted_gateway: String,
    },
    /// Start the Orchestration Gateway
    Gateway {
        #[arg(long, default_value = "3000")]
        port: u16,
        /// Hex-encoded Ed25519 public key authorized to submit jobs
        #[arg(long)]
        admission_public_key: String,
    },
    /// Generate a private key authorized to submit jobs to an admission-protected gateway
    GenerateAdmissionKey {
        /// Destination for the 32-byte Ed25519 private key
        #[arg(long, default_value = ".swarm_admission.key")]
        output: String,
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
        /// Path to the 32-byte Ed25519 private key authorized to submit jobs
        #[arg(long)]
        admission_private_key: String,
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
    runtime: RuntimeKind,
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

    if let Commands::GenerateAdmissionKey { output } = &cli.command {
        generate_admission_key(output)?;
        return Ok(());
    }

    if cli.is_node_command() {
        println!(
            "🐝 Swarm Runtime v{} - Initializing...",
            env!("CARGO_PKG_VERSION")
        );
    }

    // Unified Cryptographic Identity Loading
    let id_path = ".swarm_identity";
    let signing_key = if let Ok(bytes) = fs::read(id_path) {
        let key_bytes: [u8; 32] = bytes.try_into().map_err(|_| {
            anyhow::anyhow!(
                "Invalid cryptographic identity at {id_path}: expected exactly 32 bytes"
            )
        })?;
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
            std::io::Write::write_all(&mut file, &key.to_bytes())
                .context("Failed to write identity")?;
        }
        #[cfg(not(unix))]
        {
            std::fs::write(id_path, key.to_bytes()).context("Failed to write identity")?;
        }
        key
    };

    let verifying_key = signing_key.verifying_key();
    println!(
        "🔑 Node Public Key (Hex): {}",
        hex::encode(verifying_key.as_bytes())
    );
    let seed = signing_key.to_bytes();

    match &cli.command {
        Commands::Start {
            shard,
            bootnode,
            trusted_gateway,
        } => {
            // PHASE 15: Lazarus Monitoring for the Edge Worker
            let (alert_tx, mut alert_rx) = mpsc::channel::<CriticalFailure>(32);
            let alert_tx_clone = alert_tx.clone();

            let worker_shard = *shard;
            let gateway_bytes = hex::decode(trusted_gateway)
                .context("trusted_gateway must be a hexadecimal Ed25519 public key")?;
            let gateway_key_bytes: [u8; 32] = gateway_bytes
                .as_slice()
                .try_into()
                .map_err(|_| anyhow::anyhow!("trusted_gateway must contain exactly 32 bytes"))?;
            let worker_key = ed25519_dalek::VerifyingKey::from_bytes(&gateway_key_bytes)
                .context("trusted_gateway is not a valid Ed25519 public key")?;
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
        Commands::Gateway {
            port,
            admission_public_key,
        } => {
            let admission_key_bytes = hex::decode(admission_public_key)
                .context("admission_public_key must be hexadecimal")?;
            let admission_key_bytes: [u8; 32] =
                admission_key_bytes.as_slice().try_into().map_err(|_| {
                    anyhow::anyhow!("admission_public_key must contain exactly 32 bytes")
                })?;
            let admission_key = VerifyingKey::from_bytes(&admission_key_bytes)
                .context("admission_public_key is not a valid Ed25519 public key")?;

            // PHASE 15: Lazarus Monitoring for the Orchestration Gateway
            let (alert_tx, mut alert_rx) = mpsc::channel::<CriticalFailure>(32);
            let alert_tx_clone = alert_tx.clone();

            let gw_port = *port;
            let gw_key = signing_key.clone();

            tokio::spawn(async move {
                if let Err(e) = gateway::run_gateway(gw_port, gw_key, admission_key).await {
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
            admission_private_key,
        } => {
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .context("Failed to create HTTP client")?;
            let admission_signing_key = load_admission_signing_key(admission_private_key)?;
            println!("🚀 Preparing deployment for: {}", file);

            let meta = fs::metadata(file).context("Failed to read file metadata")?;
            if meta.len() > 50 * 1024 * 1024 {
                anyhow::bail!("SECURITY ALARM: Payload exceeds 50MB limit. Deployment aborted to prevent OOM.");
            }
            let (wasm_bytes, dataset, runtime) = match lang.to_lowercase().as_str() {
                "wasm" => {
                    let wasm_bytes = fs::read(file).context("Failed to read .wasm file")?;
                    if !wasm_bytes.starts_with(b"\0asm") {
                        anyhow::bail!("SECURITY ALARM: Invalid WASM magic number. File is corrupted or malicious.");
                    }
                    (
                        wasm_bytes,
                        vec!["EXECUTE_NATIVE_WASM".to_string()],
                        RuntimeKind::Wasm,
                    )
                }
                "python" | "js" | "javascript" | "lua" | "ruby" | "rb" | "php" | "sqlite"
                | "sql" => {
                    let code = fs::read_to_string(file)
                        .with_context(|| format!("Failed to read file: {}", file))?;
                    let runtime = match lang.to_lowercase().as_str() {
                        "python" => RuntimeKind::Python,
                        "js" | "javascript" => RuntimeKind::JavaScript,
                        "lua" => RuntimeKind::Lua,
                        "ruby" | "rb" => RuntimeKind::Ruby,
                        "php" => RuntimeKind::Php,
                        "sqlite" | "sql" => RuntimeKind::Sqlite,
                        _ => unreachable!(),
                    };
                    (Vec::new(), vec![code], runtime)
                }
                _ => anyhow::bail!(
                    "Unsupported language: {}. Supported: python, js, lua, ruby, php, sqlite, wasm",
                    lang
                ),
            };

            let metadata = DeployMetadata { dataset, runtime };
            let metadata_json = serde_json::to_string(&metadata)?;
            let gw_list: Vec<&str> = gateways.split(',').map(|s| s.trim()).collect();
            let mut success = false;

            for gw in gw_list {
                let wasm_part = reqwest::multipart::Part::bytes(wasm_bytes.clone())
                    .file_name(file.clone())
                    .mime_str("application/wasm")?;

                let metadata_part = reqwest::multipart::Part::text(metadata_json.clone())
                    .mime_str("application/json")?;

                let expires_at = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|duration| duration.as_secs())
                    .unwrap_or(0)
                    + 300;
                let nonce = Uuid::new_v4();
                let signature = admission_signing_key.sign(&admission_message(
                    &wasm_bytes,
                    metadata_json.as_bytes(),
                    &nonce,
                    expires_at,
                ));
                let admission_json = serde_json::to_string(&AdmissionToken {
                    nonce,
                    expires_at,
                    signature: signature.to_bytes().to_vec(),
                })?;
                let admission_part =
                    reqwest::multipart::Part::text(admission_json).mime_str("application/json")?;

                let form = reqwest::multipart::Form::new()
                    .part("wasm", wasm_part)
                    .part("metadata", metadata_part)
                    .part("admission", admission_part);
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
        Commands::GenerateAdmissionKey { .. } => {
            unreachable!("handled before identity initialization")
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
                        let filename =
                            format!("download_{}.bin", hash.get(..8).unwrap_or(hash.as_str()));
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

fn generate_admission_key(path: &str) -> Result<()> {
    let mut csprng = OsRng;
    let key = SigningKey::generate(&mut csprng);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(path)
            .with_context(|| format!("Failed to create admission key: {path}"))?;
        std::io::Write::write_all(&mut file, &key.to_bytes())
            .context("Failed to write admission key")?;
    }
    #[cfg(not(unix))]
    {
        if std::path::Path::new(path).exists() {
            anyhow::bail!("Refusing to overwrite existing admission key: {path}");
        }
        std::fs::write(path, key.to_bytes()).context("Failed to write admission key")?;
    }

    println!("Created admission private key: {path}");
    println!(
        "Admission public key (pass to `swarm-node gateway --admission-public-key`): {}",
        hex::encode(key.verifying_key().as_bytes())
    );
    Ok(())
}

fn load_admission_signing_key(path: &str) -> Result<SigningKey> {
    let key_bytes =
        fs::read(path).with_context(|| format!("Failed to read admission private key: {path}"))?;
    let key_bytes: [u8; 32] = key_bytes.try_into().map_err(|_| {
        anyhow::anyhow!("Admission private key at {path} must contain exactly 32 bytes")
    })?;
    Ok(SigningKey::from_bytes(&key_bytes))
}

impl Cli {
    fn is_node_command(&self) -> bool {
        matches!(
            self.command,
            Commands::Start { .. } | Commands::Gateway { .. }
        )
    }
}
