use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;

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
struct DeployPayload {
    wasm_base64: String,
    dataset: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct JobStatusResponse {
    status: String,
    total_sum: i32,
    breakdown: Vec<(u32, i32)>,
    hashes: Vec<(u32, String)>,
    missing_shards: Vec<u32>,
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
        } => {
            println!("🚀 Preparing deployment for: {}", file);

            let (wasm_base64, dataset) = match lang.to_lowercase().as_str() {
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

                    use base64::{Engine as _, engine::general_purpose};
                    let encoded = general_purpose::STANDARD.encode(&wasm_bytes);
                    (encoded, vec!["EXECUTE_NATIVE_WASM".to_string()])
                },
                "wasm" => {
                    println!("⚙️  Reading raw WebAssembly binary...");
                    let wasm_bytes = fs::read(file).context("Failed to read .wasm file")?;
                    use base64::{Engine as _, engine::general_purpose};
                    let encoded = general_purpose::STANDARD.encode(&wasm_bytes);
                    (encoded, vec!["EXECUTE_NATIVE_WASM".to_string()])
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

                    use base64::{Engine as _, engine::general_purpose};
                    let encoded = general_purpose::STANDARD.encode(&wasm_bytes);
                    (encoded, vec!["EXECUTE_NATIVE_WASM".to_string()])
                },
                "python" | "js" | "javascript" | "lua" | "ruby" | "rb" | "php" | "sqlite" | "sql" => {
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
                    (identifier.to_string(), vec![code])
                },
                _ => anyhow::bail!("Unsupported language: {}. Currently supported: python, js, lua, ruby, php, sqlite, go, wasm, zig", lang),
            };

            let payload = DeployPayload {
                wasm_base64,
                dataset,
            };

            let url = format!("{}/api/v1/jobs", gateway.trim_end_matches('/'));
            println!("📡 Dispatching payload to Gateway at {}...", url);

            let res = client
                .post(&url)
                .json(&payload)
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
                            let filename = format!("download_{}.bin", &hash[..8]);
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
