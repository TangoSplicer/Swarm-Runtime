use syn::{ItemStruct, Item, Fields};
use anyhow::Result; // FIX: Removed unused 'Context'
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
    let mut s = DefaultHasher::new();
    name.hash(&mut s);
    s.finish() % shard_count
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
