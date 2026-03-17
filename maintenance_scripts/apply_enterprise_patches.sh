#!/bin/bash

echo "🚀 Applying Phase 12 Enterprise Hardening Patches..."

# ==========================================
# 1. LAZARUS: Exponential Backoff & Arc Mutability
# ==========================================
cat << 'EOF' > components/lazarus/src/lib.rs
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
EOF
echo "✅ Lazarus patched: Decoupled mutability and added exponential backoff."

# ==========================================
# 2. SYNAPSE: Network Resilience & Strict Crypto
# ==========================================
# Reduce 300s timeout down to 15s to prevent asynchronous memory leaks
sed -i 's/with_request_timeout(std::time::Duration::from_secs(300))/with_request_timeout(std::time::Duration::from_secs(15))/g' components/synapse/src/lib.rs
sed -i 's/with_request_timeout(Duration::from_secs(300))/with_request_timeout(std::time::Duration::from_secs(15))/g' components/synapse/src/lib.rs

# Enforce Strict Validation Mode to prevent BFT poisoning
sed -i 's/validation_mode(gossipsub::ValidationMode::Permissive)/validation_mode(gossipsub::ValidationMode::Strict)/g' components/synapse/src/lib.rs

echo "✅ Synapse patched: Locked down Gossipsub crypto and enforced 15s timeouts."

# ==========================================
# 3. PRISM: Deterministic BFT Sharding (SHA-256)
# ==========================================
cat << 'EOF' > components/prism/src/sharder.rs
use syn::{ItemStruct, Item, Fields};
use anyhow::Result;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct StateField {
    pub name: String,
    pub type_name: String,
    pub assigned_shard: u64,
}

#[derive(Debug)]
pub struct StateManifest {
    pub fields: Vec<StateField>,
    pub shard_count: u64,
}

fn assign_shard(name: &str, shard_count: u64) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    let result = hasher.finalize();
    
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&result[0..8]);
    let deterministic_hash = u64::from_be_bytes(bytes);
    
    deterministic_hash % shard_count
}

pub fn extract_state(ast: &syn::File, shard_count: u64) -> Result<StateManifest> {
    let mut fields = Vec::new();

    for item in &ast.items {
        if let Item::Struct(ItemStruct { ident, fields: struct_fields, .. }) = item {
            if ident == "State" {
                if let Fields::Named(named_fields) = struct_fields {
                    for field in &named_fields.named {
                        let field_name = field.ident.as_ref().unwrap().to_string();
                        let type_name = match &field.ty {
                            syn::Type::Path(p) => p.path.segments.last().unwrap().ident.to_string(),
                            _ => "Unknown".to_string(),
                        };

                        let shard_id = assign_shard(&field_name, shard_count);

                        fields.push(StateField {
                            name: field_name,
                            type_name,
                            assigned_shard: shard_id,
                        });
                    }
                }
            }
        }
    }

    if fields.is_empty() {
        anyhow::bail!("Sharding Error: No 'struct State' found in source code.");
    }

    Ok(StateManifest { fields, shard_count })
}
EOF

# Inject the sha2 dependency into Prism's Cargo.toml if it doesn't already exist
if ! grep -q 'sha2' components/prism/Cargo.toml; then
    sed -i '/\[dependencies\]/a sha2 = "0.10"' components/prism/Cargo.toml
fi

echo "✅ Prism patched: DefaultHasher eradicated, SHA-256 deterministic sharding injected."

echo "🎉 All Enterprise patches applied successfully."
