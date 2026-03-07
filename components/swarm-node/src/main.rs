use anyhow::Result;
use clap::{Parser, Subcommand};
use ed25519_dalek::{SigningKey, VerifyingKey};
use std::fs;
use std::io::{Read, Write};

mod types;
mod gateway;
mod worker;

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

// Phase 9.1: Cryptographic Identity Generation
fn get_or_create_identity() -> Result<[u8; 32]> {
    let path = ".swarm_identity";
    
    // 1. Try to load an existing identity
    if let Ok(mut file) = fs::File::open(path) {
        let mut seed = [0u8; 32];
        file.read_exact(&mut seed)?;
        println!("🔑 Loaded existing cryptographic identity from {}", path);
        return Ok(seed);
    }

    // 2. If missing, generate a new secure identity natively via the OS
    println!("🌱 Generating new cryptographic identity...");
    let mut seed = [0u8; 32];
    let mut urandom = fs::File::open("/dev/urandom").expect("OS must provide /dev/urandom");
    urandom.read_exact(&mut seed)?;

    // 3. Save it permanently so the PeerId remains consistent across reboots
    let mut file = fs::File::create(path)?;
    file.write_all(&seed)?;
    println!("💾 Saved new identity to {}", path);

    Ok(seed)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Establish the Cryptographic Root of Trust dynamically
    let seed = get_or_create_identity()?;
    let signing_key = SigningKey::from_bytes(&seed);
    let verifying_key = VerifyingKey::from(&signing_key);

    match cli.command {
        Commands::Gateway { port } => {
            gateway::run_gateway(port, signing_key).await?;
        }
        Commands::Start { shard } => {
            let shard_id = shard.unwrap_or(1);
            worker::run_worker(shard_id, verifying_key, seed).await?;
        }
    }

    Ok(())
}
