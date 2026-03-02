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
    }
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
        Commands::Deploy { file, lang, gateway } => {
            println!("🚀 Preparing deployment for: {}", file);

            let code = fs::read_to_string(file)
                .with_context(|| format!("Failed to read file: {}", file))?;

            let identifier = match lang.to_lowercase().as_str() {
                "python" => "POLYGLOT:PYTHON",
                "js" | "javascript" => "POLYGLOT:JS",
                _ => anyhow::bail!("Unsupported language: {}. Currently supported: python, js", lang),
            };

            let payload = DeployPayload {
                wasm_base64: identifier.to_string(),
                dataset: vec![code],
            };

            let url = format!("{}/api/v1/jobs", gateway.trim_end_matches('/'));
            println!("📡 Dispatching payload to Gateway at {}...", url);

            let res = client.post(&url)
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
        Commands::Status { job_id, gateway } => {
            let url = format!("{}/api/v1/jobs/{}", gateway.trim_end_matches('/'), job_id);
            println!("🔍 Querying Gateway for Job ID: {}...", job_id);

            let res = client.get(&url)
                .send()
                .await
                .context("Failed to connect to the Swarm Gateway.")?;

            if res.status().is_success() {
                let status_data: JobStatusResponse = res.json().await?;
                
                println!("\\n=== 📊 Swarm Job Status ===");
                println!("Status:          {}", status_data.status.to_uppercase());
                
                if status_data.status == "completed" {
                    println!("Consensus Hash:  {}", status_data.hashes.first().map(|(_, h)| h.as_str()).unwrap_or("NONE"));
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
