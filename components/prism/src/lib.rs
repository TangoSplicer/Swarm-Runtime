//! # Prism Compiler
//!
//! The `prism` crate is responsible for parsing raw Rust source code and preparing it
//! for distributed execution across the Swarm network.

use anyhow::{Result, Context};
use syn::parse_file;

pub mod sharder;

/// The Core Prism Compiler Struct.
///
/// This is the entry point for all code analysis.
///
/// # Example
/// ```
/// use prism::PrismCompiler;
/// let compiler = PrismCompiler::new();
/// let code = r#"
///     struct State {
///         users: Vec<String>,
///         products: Vec<u64>
///     }
///     fn main() {}
/// "#;
/// // Compile with 2 shards
/// compiler.transpile(code, 2).unwrap();
/// ```
pub struct PrismCompiler;

impl PrismCompiler {
    pub fn new() -> Self {
        PrismCompiler
    }

    /// Entry point: Accepts raw Rust source code and verifies it.
    ///
    /// # Arguments
    /// * `source_code` - A string slice containing the Rust code.
    /// * `shard_count` - How many pieces to split the state into.
    pub fn transpile(&self, source_code: &str, shard_count: u64) -> Result<String> {
        // Step 1: Parse AST
        let ast = parse_file(source_code)
            .context("Prism: Failed to parse source code.")?;

        // Step 2: Identify State & Assign Shards
        let manifest = sharder::extract_state(&ast, shard_count)?;

        // Step 3: Generate Wasm (Placeholder with Manifest)
        self.generate_flight_plan(&manifest)
    }

    /// Internal function to generate the execution plan
    fn generate_flight_plan(&self, manifest: &sharder::StateManifest) -> Result<String> {
        let mut output = String::new();
        output.push_str(&format!("// SWARM FLIGHT PLAN (Shards: {})\n", manifest.shard_count));
        output.push_str("// -----------------------------\n");
        
        for field in &manifest.fields {
            output.push_str(&format!("// Field '{}' ({}) -> Shard {}\n", 
                field.name, field.type_name, field.assigned_shard));
        }
        
        Ok(output)
    }
}
