use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ed25519_dalek::{Signer, SigningKey};
use serde::{Deserialize, Serialize};
use std::fs;
use synapse::{admission_message, AdmissionToken};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "swarm")]
#[command(about = "Swarm Runtime CLI - Deploy and monitor code on the decentralized mesh", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch a file from the network using its SHA-256 Hash
    Fetch {
        /// The SHA-256 Hash of the file
        hash: String,

        /// The Gateway URL
        #[arg(long, default_value = "http://127.0.0.1:3000")]
        gateway: String,
    },
    /// Deploy a script to the Swarm network
    Deploy {
        /// The path to the script file (e.g., app.py)
        file: String,

        /// The language of the script (e.g., python)
        #[arg(short, long, default_value = "python")]
        lang: String,

        /// The Gateway URL
        #[arg(long, default_value = "http://127.0.0.1:3000")]
        gateway: String,

        /// Path to the 32-byte Ed25519 private key authorized to submit jobs
        #[arg(long)]
        admission_private_key: String,
    },
    /// Check the status and output of a deployed job
    Status {
        /// The Job ID returned from the deploy command
        job_id: String,

        /// The Gateway URL
        #[arg(long, default_value = "http://127.0.0.1:3000")]
        gateway: String,
    },
}

#[derive(Serialize)]
struct DeployMetadata {
    dataset: Vec<String>,
    runtime: RuntimeKind,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum RuntimeKind {
    Wasm,
    Python,
    JavaScript,
    Lua,
    Ruby,
    Php,
    Sqlite,
}

#[derive(Deserialize, Debug)]
struct JobStatusResponse {
    status: String,
    total_sum: i32,
    breakdown: Vec<(u32, i32)>,
    hashes: Vec<(u32, String)>,
    missing_shards: Vec<u32>,
}

fn load_admission_signing_key(path: &str) -> Result<SigningKey> {
    let key_bytes =
        fs::read(path).with_context(|| format!("Failed to read admission private key: {path}"))?;
    let key_bytes: [u8; 32] = key_bytes.try_into().map_err(|_| {
        anyhow::anyhow!("Admission private key at {path} must contain exactly 32 bytes")
    })?;
    Ok(SigningKey::from_bytes(&key_bytes))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match &cli.command {
        Commands::Deploy {
            file,
            lang,
            gateway,
            admission_private_key,
        } => {
            let admission_signing_key = load_admission_signing_key(admission_private_key)?;
            println!("🚀 Preparing deployment for: {}", file);

            let (wasm_bytes, dataset, runtime) = match lang.to_lowercase().as_str() {
                                                "zig" => {
                    println!("⚙️  Locally compiling Zig source natively to WASIp1...");
                    let temp_wasm = format!("{}.wasm", file.replace(".zig", "").replace("/", "_"));

                    let status = std::process::Command::new("zig")
                        .args([
                            "build-exe", file,
                            "-target", "wasm32-wasi",
                            "-O", "ReleaseSmall",
                            &format!("-femit-bin={}", temp_wasm)
                        ])
                        .status()
                        .context("Failed to execute zig compiler. Is Zig installed?")?;

                    if !status.success() {
                        anyhow::bail!("❌ Zig compilation failed!");
                    }

                    let wasm_bytes = std::fs::read(&temp_wasm).context("Failed to read compiled wasm")?;
                    let _ = std::fs::remove_file(&temp_wasm); // Cleanup binary
                    let _ = std::fs::remove_file(format!("{}.o", temp_wasm)); // Cleanup object file

                    (
                        wasm_bytes,
                        vec!["EXECUTE_NATIVE_WASM".to_string()],
                        RuntimeKind::Wasm,
                    )
                },
                "wasm" => {
                    println!("⚙️  Reading raw WebAssembly binary...");
                    let wasm_bytes = fs::read(file).context("Failed to read .wasm file")?;
                    (
                        wasm_bytes,
                        vec!["EXECUTE_NATIVE_WASM".to_string()],
                        RuntimeKind::Wasm,
                    )
                },
                "go" => {
                    println!("⚙️  Locally compiling Go source natively to WASIp1...");
                    let temp_wasm = format!("{}.wasm", file.replace(".go", "").replace("/", "_"));
                    let status = std::process::Command::new("go")
                        .env("GOOS", "wasip1")
                        .env("GOARCH", "wasm")
                        .args(["build", "-ldflags=-s -w", "-o", &temp_wasm, file])
                        .status()
                        .context("Failed to execute go compiler. Is Go installed?")?;

                    if !status.success() {
                        anyhow::bail!("❌ Go compilation failed!");
                    }

                    let wasm_bytes = fs::read(&temp_wasm).context("Failed to read compiled wasm")?;
                    let _ = fs::remove_file(&temp_wasm); // Cleanup

                    (
                        wasm_bytes,
                        vec!["EXECUTE_NATIVE_WASM".to_string()],
                        RuntimeKind::Wasm,
                    )
                },
                "python" | "js" | "javascript" | "lua" | "ruby" | "rb" | "php" | "sqlite" | "sql" => {
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
                },
                _ => anyhow::bail!("Unsupported language: {}. Currently supported: python, js, lua, ruby, php, sqlite, go, wasm, zig", lang),
            };

            let metadata_json = serde_json::to_string(&DeployMetadata { dataset, runtime })?;
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

            let wasm_part = reqwest::multipart::Part::bytes(wasm_bytes)
                .file_name(file.clone())
                .mime_str("application/wasm")?;
            let metadata_part =
                reqwest::multipart::Part::text(metadata_json).mime_str("application/json")?;
            let admission_part =
                reqwest::multipart::Part::text(admission_json).mime_str("application/json")?;
            let form = reqwest::multipart::Form::new()
                .part("wasm", wasm_part)
                .part("metadata", metadata_part)
                .part("admission", admission_part);

            let url = format!("{}/api/v1/jobs", gateway.trim_end_matches('/'));
            println!("📡 Dispatching payload to Gateway at {}...", url);

            let res = client
                .post(&url)
                .multipart(form)
                .send()
                .await
                .context("Failed to connect to the Swarm Gateway.")?;

            if res.status().is_success() {
                let response_text = res.text().await?;
                println!("✅ Deployment Successful!");
                println!("   Gateway Response: {}", response_text);
                println!("   (Run 'swarm status <JOB_ID>' to check execution results)");
            } else {
                println!("❌ Deployment Failed (Status: {})", res.status());
                println!("   Error: {}", res.text().await.unwrap_or_default());
            }
        }
        Commands::Fetch { hash, gateway } => {
            println!("🔍 Querying Gateway for File Hash: {}...", hash);

            match client
                .get(format!("{}/api/v1/data/{}", gateway, hash))
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        if let Ok(bytes) = response.bytes().await {
                            let filename =
                                format!("download_{}.bin", hash.get(..8).unwrap_or(hash.as_str()));
                            if std::fs::write(&filename, &bytes).is_ok() {
                                println!("✅ Success! File downloaded to: {}", filename);
                            } else {
                                println!("❌ Failed to write file to disk.");
                            }
                        }
                    } else {
                        println!("❌ Failed to retrieve file. It may not exist on the network.");
                    }
                }
                Err(e) => println!("❌ Failed to connect to Gateway: {}", e),
            }
        }
        Commands::Status { job_id, gateway } => {
            let url = format!("{}/api/v1/jobs/{}", gateway.trim_end_matches('/'), job_id);
            println!("🔍 Querying Gateway for Job ID: {}...", job_id);

            let res = client
                .get(&url)
                .send()
                .await
                .context("Failed to connect to the Swarm Gateway.")?;

            if res.status().is_success() {
                let status_data: JobStatusResponse = res.json().await?;

                println!("\\n=== 📊 Swarm Job Status ===");
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
                    println!("Shard Breakdown: {:?}", status_data.breakdown);
                    println!("Verified Shards: {}", status_data.hashes.len());
                } else {
                    println!("Missing Shards:  {:?}", status_data.missing_shards);
                }
                println!("===========================\\n");
            } else {
                println!("❌ Failed to retrieve status (HTTP {})", res.status());
            }
        }
    }

    Ok(())
}
