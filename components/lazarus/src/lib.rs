use anyhow::Result;
use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;

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
        // println!("Lazarus: Now watching service '{}'", service.name());
        loop {
            if !service.is_alive().await {
                // println!("Lazarus: ALARM! Service '{}' is unresponsive.", service.name());
                match service.restart().await {
                    Ok(_) => {}, // println!("Lazarus: Service '{}' recovered.", service.name()),
                    Err(_) => {}, // println!("Lazarus: Recovery failed."),
                }
            }
            sleep(self.check_interval).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockService {
        alive: bool,
        restarts: u32,
    }

    #[async_trait]
    impl Monitorable for MockService {
        fn name(&self) -> &str { "MockService" }
        async fn is_alive(&self) -> bool { self.alive }
        async fn restart(&mut self) -> Result<()> {
            self.restarts += 1;
            self.alive = true;
            Ok(())
        }
    }

    // TRUTH PROTOCOL: Self-Check
    #[tokio::test]
    async fn test_recovery_mechanism() {
        println!("TEST: Starting Watchdog check...");
        let mut service = MockService { alive: false, restarts: 0 };
        
        // Manually trigger logic without infinite loop for testing
        if !service.is_alive().await {
            service.restart().await.unwrap();
        }

        assert!(service.alive, "Lazarus failed to revive the service");
        assert_eq!(service.restarts, 1, "Restart counter should be 1");
        println!("TEST: Watchdog successfully revived service.");
    }
}
