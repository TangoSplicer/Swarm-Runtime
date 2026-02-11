use anyhow::Result;
use log::{info, error};
use std::time::Duration;

use judge::Judge;
use synapse::SynapseNode;
use lazarus::{Lazarus, Monitorable};
use async_trait::async_trait;

struct ManagedNode {
    inner: SynapseNode,
    running: bool,
}

// FIX: Added #[async_trait] here to match the trait definition
#[async_trait]
impl Monitorable for ManagedNode {
    fn name(&self) -> &str { "Synapse P2P Layer" }

    async fn is_alive(&self) -> bool {
        self.running
    }

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
    println!("=== Swarm Runtime v0.1.0 ===");
    println!("System: Boot sequence initiated...");

    println!("System: Checking Prism Compiler...");
    let _compiler = prism::PrismCompiler::new();
    println!("System: Prism is online.");

    println!("System: Initializing Judge Hypervisor...");
    let _judge = Judge::new()?;
    println!("System: Judge is secure and ready.");

    println!("System: Booting Synapse P2P Node...");
    let p2p_node = SynapseNode::new().await?;
    
    // Create the managed node
    // Note: We move this into the spawn block to satisfy ownership rules
    // or we wrap it in a Mutex if we want to share it. 
    // For this simple example, we will move it into the watchdog loop.
    let mut managed_node = ManagedNode { 
        inner: p2p_node, 
        running: true 
    };
    println!("System: Node is live! Peer ID: {}", managed_node.inner.peer_id);

    println!("System: Spawning Lazarus Watchdog...");
    
    // Spawn the watchdog
    tokio::spawn(async move {
        let watchdog = Lazarus::new(5);
        if let Err(e) = watchdog.watch(&mut managed_node).await {
            error!("Watchdog Failed: {}", e);
        }
    });

    println!("=== System Fully Operational ===");
    
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
