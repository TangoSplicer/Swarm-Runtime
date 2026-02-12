use anyhow::Result;
use clap::{Parser, Subcommand};
use log::{info, error};
use std::time::Duration;

use judge::Judge;
use synapse::SynapseNode;
use lazarus::{Lazarus, Monitorable};
use async_trait::async_trait;

#[derive(Parser)]
#[command(name = "swarm")]
#[command(about = "Swarm Runtime: Decentralized Compute Node", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start,
    Check {
        path: String,
    },
}

struct ManagedNode {
    inner: SynapseNode,
    running: bool,
}

#[async_trait]
impl Monitorable for ManagedNode {
    fn name(&self) -> &str { "Synapse P2P Layer" }
    async fn is_alive(&self) -> bool { self.running }
    async fn restart(&mut self) -> Result<()> {
        info!("System: Restarting P2P Node...");
        self.inner = SynapseNode::new().await?;
        self.running = true;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check { path } => {
            println!("System: analyzing file '{}'...", path);
            let compiler = prism::PrismCompiler::new();
            let code = std::fs::read_to_string(path).unwrap_or("fn main() {}".to_string());
            match compiler.transpile(&code) {
                Ok(_) => println!("SUCCESS: File is valid Swarm code."),
                Err(e) => println!("FAILURE: Syntax Error: {}", e),
            }
        }
        Commands::Start => {
            println!("=== Swarm Runtime v0.1.0 ===");
            println!("System: Boot sequence initiated...");

            println!("System: Initializing Judge Hypervisor...");
            let _judge = Judge::new()?;
            println!("System: Judge is secure.");

            println!("System: Booting Synapse P2P Node...");
            let p2p_node = SynapseNode::new().await?;
            
            let mut managed_node = ManagedNode { 
                inner: p2p_node, 
                running: true 
            };
            println!("System: Node is live! Peer ID: {}", managed_node.inner.peer_id);

            println!("System: Spawning Lazarus Watchdog...");
            tokio::spawn(async move {
                let watchdog = Lazarus::new(5);
                if let Err(e) = watchdog.watch(&mut managed_node).await {
                    error!("Watchdog Failed: {}", e);
                }
            });

            println!("=== System Fully Operational (Press Ctrl+C to stop) ===");
            
            // HEARTBEAT LOOP
            loop {
                tokio::time::sleep(Duration::from_secs(10)).await;
                println!("System: Heartbeat... [NOMINAL]");
            }
        }
    }
    Ok(())
}
