use anyhow::{anyhow, Result};
use std::sync::Arc;
use wasmer::{
    imports, CompilerConfig, EngineBuilder, Instance, Module, Store, Memory, MemoryType, Value
};
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::Metering;
use wasmer_middlewares::metering::{get_remaining_points, set_remaining_points, MeteringPoints};

pub struct Judge {
    store: Store,
    gas_limit: u64,
}

impl Judge {
    /// Creates a new Judge instance with Gas Metering and Singlepass compilation.
    pub fn new(gas_limit: Option<u64>) -> Result<Self> {
        let limit = gas_limit.unwrap_or(5_000_000);

        // 1. Define the cost of each Wasm instruction (1 instruction = 1 gas)
        let cost_function = |_: &wasmer::wasmparser::Operator| -> u64 { 1 };
        
        // 2. Initialize the Metering Middleware
        let metering = Arc::new(Metering::new(limit, cost_function));
        
        // 3. Attach it to the Singlepass Compiler
        let mut compiler = Singlepass::default();
        compiler.push_middleware(metering);
        
        // 4. Build the Secure Engine
        let engine = EngineBuilder::new(compiler);
        let store = Store::new(engine);

        Ok(Self { store, gas_limit: limit })
    }

    /// Executes the Wasm module against the dataset safely.
    pub fn execute(&mut self, wasm_bytes: &[u8], dataset: &[i32]) -> Result<i32> {
        // Reset gas for this execution run
        let module = Module::new(&self.store, wasm_bytes)?;
        
        // Create an isolated linear memory block for this job
        let memory = Memory::new(&mut self.store, MemoryType::new(1, None, false))?;

        // Write the dataset into the WebAssembly linear memory
        let view = memory.view(&self.store);
        for (i, &val) in dataset.iter().enumerate() {
            let offset = (i * 4) as u64; // 4 bytes per i32
            let bytes = val.to_le_bytes();
            view.write(offset, &bytes).map_err(|e| anyhow!("Memory Access Error: {}", e))?;
        }

        // Define the imports (Inject memory and host functions)
        let import_object = imports! {
            "env" => {
                "memory" => memory.clone(),
            }
        };

        let instance = Instance::new(&mut self.store, &module, &import_object)?;
        
        // We must manually apply the gas limit to the new instance
        set_remaining_points(&mut self.store, &instance, self.gas_limit);

        // Auto-detect the execution function
        let execute_func = instance.exports.get_function("execute")?;

        // Pass the array length (item count) to the Wasm function
        let args = [Value::I32(dataset.len() as i32)];
        
        println!("⛽ Executing Wasm Sandbox (Gas Limit: {})", self.gas_limit);
        
        // Execute! If it hits an infinite loop, it will return an Error here.
        match execute_func.call(&mut self.store, &args) {
            Ok(result) => {
                if let Some(Value::I32(val)) = result.get(0) {
                    let remaining = get_remaining_points(&mut self.store, &instance);
                    if let MeteringPoints::Remaining(points) = remaining {
                        println!("✅ Execution complete. Gas used: {}", self.gas_limit - points);
                    }
                    Ok(*val)
                } else {
                    Err(anyhow!("Wasm function did not return an i32"))
                }
            },
            Err(e) => {
                // Check if it was a gas exhaustion trap
                if let MeteringPoints::Exhausted = get_remaining_points(&mut self.store, &instance) {
                    println!("🚨 SANDBOX TRAP: Out of Gas! Terminating infinite loop.");
                    Err(anyhow!("Gas Exhaustion: Wasm module exceeded {} instructions.", self.gas_limit))
                } else {
                    Err(anyhow!("Wasm Runtime Error: {}", e))
                }
            }
        }
    }
}
