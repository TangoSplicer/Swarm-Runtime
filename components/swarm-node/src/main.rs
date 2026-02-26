use anyhow::Result;
use clap::{Parser, Subcommand};
use ed25519_dalek::{SigningKey, VerifyingKey};

mod types;
mod gateway;
mod worker;

use types::GATEWAY_SECRET_SEED;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start { #[arg(long)] shard: Option<u64> },
    Gateway { #[arg(long, default_value = "3000")] port: u16 },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Establish the Cryptographic Root of Trust for the entire node
    let signing_key = SigningKey::from_bytes(&GATEWAY_SECRET_SEED);
    let verifying_key = VerifyingKey::from(&signing_key);
    
    match cli.command {
        Commands::Gateway { port } => {
            gateway::run_gateway(port, signing_key).await?;
        }
        Commands::Start { shard } => {
            let shard_id = shard.unwrap_or(1);
            worker::run_worker(shard_id, verifying_key).await?;
        }
    }
    
    Ok(())
}
