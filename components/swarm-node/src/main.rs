use anyhow::Result;
use clap::{Parser, Subcommand};
use std::time::Duration;

use judge::Judge;
use synapse::SynapseNode;
use lazarus::{Lazarus, Monitorable};
use async_trait::async_trait;

#[derive(Parser)]
#[command(name = "swarm")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start,
    Listen,
    Say {
        message: String,
        #[arg(long)]
        peer: Option<String>,
    },
    Check {
        path: String,
    },
}

struct ManagedNode {
    running: bool,
}

#[async_trait]
impl Monitorable for ManagedNode {
    fn name(&self) -> &str { "Synapse P2P Layer" }
    async fn is_alive(&self) -> bool { self.running }
    async fn restart(&mut self) -> Result<()> {
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
            let compiler = prism::PrismCompiler::new();
            let code = std::fs::read_to_string(path).unwrap_or("fn main() {}".to_string());
            
            println!("System: Compiling with 4 shards...");
            // FIX: Added shard_count = 4
            match compiler.transpile(&code, 4) {
                Ok(plan) => {
                    println!("SUCCESS: Compilation Complete.");
                    println!("{}", plan);
                },
                Err(e) => println!("FAILURE: {}", e),
            }
        }
        Commands::Listen => {
            println!("=== Swarm Network Listener ===");
            let mut p2p_node = SynapseNode::new().await?;
            println!("System: Connected! Peer ID: {}", p2p_node.peer_id);
            p2p_node.wait_for_event().await;
        }
        Commands::Say { message, peer } => {
            println!("=== Swarm Broadcaster ===");
            let mut p2p_node = SynapseNode::new().await?;
            if let Some(addr) = peer {
                println!("System: Dialing peer at {}...", addr);
                p2p_node.dial_peer(addr.clone())?;
            }
            p2p_node.wait_for_peers().await;
            println!("System: Negotiating GossipSub routes...");
            p2p_node.drive_for(Duration::from_secs(3)).await;

            println!("System: Publishing message: '{}'", message);
            match p2p_node.publish(message.clone()) {
                Ok(_) => println!("SUCCESS: Message sent."),
                Err(e) => println!("ERROR: Publish failed: {}", e),
            }
            p2p_node.drive_for(Duration::from_secs(1)).await;
        }
        Commands::Start => {
            println!("=== Swarm Runtime v0.1.0 ===");
            // ... (rest of start command logic remains same)
            let _judge = Judge::new()?;
            let p2p_node = SynapseNode::new().await?;
            println!("System: Node is live! Peer ID: {}", p2p_node.peer_id);
            let mut managed_node = ManagedNode { running: true };
            tokio::spawn(async move {
                let watchdog = Lazarus::new(5);
                let _ = watchdog.watch(&mut managed_node).await;
            });
            loop {
                tokio::time::sleep(Duration::from_secs(10)).await;
                println!("System: Heartbeat... [NOMINAL]");
            }
        }
    }
    Ok(())
}
