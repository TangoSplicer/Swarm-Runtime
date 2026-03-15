use anyhow::{Result, bail};
use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;

#[async_trait]
pub trait Monitorable: Send + Sync {
    async fn is_alive(&self) -> bool;
    async fn restart(&self) -> Result<()>; 
    fn name(&self) -> &str;
}

pub struct Lazarus {
    check_interval: Duration,
    max_retries: u32,
}

impl Lazarus {
    pub fn new(interval_secs: u64, max_retries: u32) -> Self {
        Lazarus {
            check_interval: Duration::from_secs(interval_secs),
            max_retries,
        }
    }

    pub async fn watch(&self, service: std::sync::Arc<dyn Monitorable>) -> Result<()> {
        let mut fail_count = 0;

        loop {
            if !service.is_alive().await {
                let backoff = 2_u64.pow(fail_count);
                sleep(Duration::from_secs(backoff)).await;

                match service.restart().await {
                    Ok(_) => {
                        fail_count = 0;
                    },
                    Err(_) => {
                        fail_count += 1;
                        if fail_count >= self.max_retries {
                            bail!("CRITICAL: Service '{}' failed to recover after {} attempts.", service.name(), self.max_retries);
                        }
                    }
                }
            } else {
                fail_count = 0;
            }
            sleep(self.check_interval).await;
        }
    }
}
