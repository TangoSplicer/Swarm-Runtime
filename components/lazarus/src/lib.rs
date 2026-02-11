use anyhow::Result;
use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;

/// The Interface that any service must implement.
/// Added 'Send + Sync' so it can be moved to the Watchdog thread.
#[async_trait]
pub trait Monitorable: Send + Sync {
    async fn is_alive(&self) -> bool;
    async fn restart(&mut self) -> Result<()>;
    fn name(&self) -> &str;
}

pub struct Lazarus {
    check_interval: Duration,
}

impl Lazarus {
    pub fn new(interval_secs: u64) -> Self {
        Lazarus {
            check_interval: Duration::from_secs(interval_secs),
        }
    }

    pub async fn watch(&self, service: &mut (dyn Monitorable + Send + Sync)) -> Result<()> {
        println!("Lazarus: Now watching service '{}'", service.name());

        loop {
            if !service.is_alive().await {
                println!("Lazarus: ALARM! Service '{}' is unresponsive.", service.name());
                println!("Lazarus: Initiating recovery protocol...");
                match service.restart().await {
                    Ok(_) => println!("Lazarus: Service '{}' recovered successfully.", service.name()),
                    Err(e) => println!("Lazarus: CRITICAL FAILURE. Recovery failed: {}", e),
                }
            }
            sleep(self.check_interval).await;
        }
    }
}
