use anyhow::{Result, Context};
use syn::parse_file;

// Import our new module
pub mod sharder;

pub struct PrismCompiler;

impl PrismCompiler {
    pub fn new() -> Self {
        PrismCompiler
    }

    pub fn transpile(&self, source_code: &str) -> Result<String> {
        // Step 1: Parse AST
        let ast = parse_file(source_code)
            .context("Prism: Failed to parse source code.")?;

        // Step 2: Identify State (Sharding Phase 1)
        println!("Prism: Analyzing State for Sharding...");
        let manifest = sharder::extract_state(&ast)?;

        println!("Prism: Found State variables: {:?}", manifest.fields);

        // Step 3: Generate Wasm (Placeholder)
        // We pass the manifest so we know what to shard later
        self.generate_wasm(&manifest)
    }

    fn generate_wasm(&self, manifest: &sharder::StateManifest) -> Result<String> {
        // In the future, we use the manifest to split the logic.
        let output = format!("// Compiled Wasm with State: {:?}", manifest.fields);
        Ok(output)
    }
}
