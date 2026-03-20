use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

/// Represents a critical Swarm component that can be health-checked and restarted.
/// 
/// ⚠️ ARCHITECTURAL MANDATE: Implementations of this trait MUST NOT hold synchronous 
/// locks (e.g., `std::sync::Mutex`) across `.await` points to prevent Tokio deadlocks.
#[async_trait]
pub trait Monitorable: Send + Sync + 'static {
    async fn is_alive(&self) -> bool;
    async fn restart(&self) -> Result<()>;
    fn name(&self) -> &str;
}

/// Alert sent to the main application loop when a component completely fails.
#[derive(Debug)]
pub struct CriticalFailure {
    pub service_name: String,
    pub error_message: String,
}

pub struct Lazarus {
    check_interval: Duration,
    max_retries: u32,
    alert_tx: mpsc::Sender<CriticalFailure>,
}

impl Lazarus {
    /// Initializes a new Lazarus instance.
    /// Requires an MPSC sender to bubble up critical failures to the main orchestrator.
    pub fn new(interval_secs: u64, max_retries: u32, alert_tx: mpsc::Sender<CriticalFailure>) -> Self {
        Lazarus {
            check_interval: Duration::from_secs(interval_secs),
            max_retries,
            alert_tx,
        }
    }

    /// Spawns a dedicated background task to monitor the given service.
    /// This ensures we do not block the main thread and respect Tokio's concurrency model.
    pub fn watch(&self, service: Arc<dyn Monitorable>) {
        let interval = self.check_interval;
        let max_retries = self.max_retries;
        let alert_tx = self.alert_tx.clone();

        tokio::spawn(async move {
            let mut fail_count = 0;
            let service_name = service.name().to_string();

            loop {
                if !service.is_alive().await {
                    // Exponential backoff, capped at 2^6 (64 seconds) to prevent overflow
                    let backoff = 2_u64.pow(fail_count.min(6));
                    sleep(Duration::from_secs(backoff)).await;

                    match service.restart().await {
                        Ok(_) => {
                            println!("♻️ Lazarus: Successfully resurrected '{}'", service_name);
                            fail_count = 0;
                        },
                        Err(e) => {
                            fail_count += 1;
                            if fail_count >= max_retries {
                                let msg = format!(
                                    "CRITICAL: Service '{}' failed to recover after {} attempts. Last error: {}", 
                                    service_name, max_retries, e
                                );
                                
                                // Send the alert back to the main thread. 
                                // We ignore the result in case the main thread has intentionally dropped the receiver.
                                let _ = alert_tx.send(CriticalFailure {
                                    service_name: service_name.clone(),
                                    error_message: msg,
                                }).await;
                                
                                break; // Terminate this specific monitoring task
                            }
                        }
                    }
                } else {
                    // Reset fail count on a successful health check
                    fail_count = 0;
                }

                sleep(interval).await;
            }
        });
    }
}
