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
/// // We must include a 'struct State' for the compiler to accept the code
/// let code = r#"
///     struct State {
///         counter: u64
///     }
///     fn main() {}
/// "#;
/// compiler.transpile(code).unwrap();
/// ```
pub struct PrismCompiler;

impl PrismCompiler {
    pub fn new() -> Self {
        PrismCompiler
    }

    pub fn transpile(&self, source_code: &str) -> Result<String> {
        let ast = parse_file(source_code)
            .context("Prism: Failed to parse source code.")?;

        let manifest = sharder::extract_state(&ast)?;

        self.generate_wasm(&manifest)
    }

    fn generate_wasm(&self, manifest: &sharder::StateManifest) -> Result<String> {
        let output = format!("// Compiled Wasm with State: {:?}", manifest.fields);
        Ok(output)
    }
}
