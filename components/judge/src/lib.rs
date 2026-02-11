use anyhow::{Result, Context};
use wasmtime::{Engine, Linker, Module, Store};

pub struct Judge {
    engine: Engine,
}

impl Judge {
    /// Initialize the Secure Hypervisor
    pub fn new() -> Result<Self> {
        // Default configuration is secure by default (Sandboxed)
        let engine = Engine::default();
        Ok(Judge { engine })
    }

    /// Execute a specific Wasm module and return the result of the "main" function
    pub fn execute(&self, wasm_bytes: &[u8]) -> Result<i32> {
        println!("Judge: Preparing to execute Wasm module...");

        // 1. Create a Store (The memory space for this specific run)
        let mut store = Store::new(&self.engine, ());

        // 2. Compile the Module from bytes
        let module = Module::new(&self.engine, wasm_bytes)
            .context("Judge: Failed to compile Wasm bytes")?;

        // 3. Linker (Connects Wasm to the Host functions)
        // For now, we have no host functions (Pure Sandbox).
        let linker = Linker::new(&self.engine);

        // 4. Instantiate the Module
        let instance = linker.instantiate(&mut store, &module)
            .context("Judge: Failed to instantiate module")?;

        // 5. Find the exported function named "run" or "main"
        // In this example, we look for a function called "calculate"
        let calculate_func = instance
            .get_typed_func::<(), i32>(&mut store, "calculate")
            .context("Judge: Function 'calculate' not found in Wasm module")?;

        // 6. Execute!
        println!("Judge: Running code inside Sandbox...");
        let result = calculate_func.call(&mut store, ())?;
        
        println!("Judge: Execution finished. Result: {}", result);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TRUTH PROTOCOL: Self-Check
    // Verify that the Judge can actually run WebAssembly code.
    #[test]
    fn test_execution_integrity() {
        let judge = Judge::new().expect("Failed to create Judge");

        // This is "WebAssembly Text Format" (WAT).
        // It represents a simple module that returns the number 42.
        // (module
        //   (func (export "calculate") (result i32)
        //     i32.const 42
        //   )
        // )
        let wasm_code = r#"
            (module
                (func (export "calculate") (result i32)
                    i32.const 42
                )
            )
        "#;

        // Wasmtime can compile text format directly for testing
        let result = judge.execute(wasm_code.as_bytes());

        assert!(result.is_ok(), "Execution failed");
        assert_eq!(result.unwrap(), 42, "Judge did not return the correct value");
    }
}
