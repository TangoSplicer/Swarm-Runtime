use anyhow::{anyhow, Result};
use std::sync::Arc;
use wasmer::{
    imports, CompilerConfig, EngineBuilder, Instance, Module, Store, Memory, MemoryType, Value
};
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::Metering;
use wasmer_middlewares::metering::{get_remaining_points, set_remaining_points, MeteringPoints};
use sha2::{Sha256, Digest};

pub struct Judge {
    store: Store,
    gas_limit: u64,
}

impl Judge {
    pub fn new(gas_limit: Option<u64>) -> Result<Self> {
        let limit = gas_limit.unwrap_or(5_000_000);
        let cost_function = |_: &wasmer::wasmparser::Operator| -> u64 { 1 };
        let metering = Arc::new(Metering::new(limit, cost_function));
        let mut compiler = Singlepass::default();
        compiler.push_middleware(metering);
        
        let engine = EngineBuilder::new(compiler);
        let store = Store::new(engine);
        Ok(Self { store, gas_limit: limit })
    }

    /// Executes the Wasm module against a complex dataset.
    /// Returns a tuple: (i32_result, sha256_hash_of_state)
    pub fn execute(&mut self, wasm_bytes: &[u8], dataset: &[String]) -> Result<(i32, String)> {
        let module = Module::new(&self.store, wasm_bytes)?;
        
        // Convert the String array into a single contiguous UTF-8 byte array
        let joined_dataset = dataset.join("\n");
        let payload_bytes = joined_dataset.as_bytes();
        
        // Host-Managed Fixed Buffer strategy (Offset: 1MB)
        let ptr_offset: u64 = 1_048_576; 
        
        // 1 Wasm Page = 64KB. Ensure we allocate enough pages to cover the 1MB offset + payload size
        let required_pages = ((ptr_offset + payload_bytes.len() as u64) / 65536) + 2;
        let memory = Memory::new(&mut self.store, MemoryType::new(required_pages as u32, None, false))?;

        // Safely write the complex byte payload directly into the sandbox's virtual memory
        let view = memory.view(&self.store);
        view.write(ptr_offset, payload_bytes).map_err(|e| anyhow!("Memory Injection Error: {}", e))?;

        let import_object = imports! { "env" => { "memory" => memory.clone() } };
        let instance = Instance::new(&mut self.store, &module, &import_object)?;
        set_remaining_points(&mut self.store, &instance, self.gas_limit);

        let execute_func = instance.exports.get_function("execute")?;

        // UPGRADED ABI: Pass (pointer, length) so the Wasm module knows where its string data lives
        let args = [Value::I32(ptr_offset as i32), Value::I32(payload_bytes.len() as i32)];
        
        println!("⛽ Executing Wasm Sandbox (Injected {} bytes at ptr 0x{:X})", payload_bytes.len(), ptr_offset);
        
        match execute_func.call(&mut self.store, &args) {
            Ok(result) => {
                if let Some(Value::I32(val)) = result.get(0) {
                    let remaining = get_remaining_points(&mut self.store, &instance);
                    if let MeteringPoints::Remaining(points) = remaining {
                        println!("✅ Execution complete. Gas used: {}", self.gas_limit - points);
                    }
                    
                    // OUTPUT STATE CONSENSUS: Hash the deterministic output execution state
                    let mut hasher = Sha256::new();
                    hasher.update(val.to_le_bytes()); // In v0.19 this will hash the output memory buffer!
                    let result_hash = format!("{:x}", hasher.finalize());

                    Ok((*val, result_hash))
                } else {
                    Err(anyhow!("Wasm function did not return an i32"))
                }
            },
            Err(e) => {
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
