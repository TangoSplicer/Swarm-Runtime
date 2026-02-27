use anyhow::{anyhow, Result};
use wasmi::*;
use wasmi_wasi::WasiCtxBuilder;
use sha2::{Sha256, Digest};
use std::fs;

pub struct Judge {
    engine: Engine,
    linker: Linker<wasmi_wasi::WasiCtx>,
}

impl Judge {
    pub fn new(_gas_limit: Option<u64>) -> Result<Self> {
        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);
        let linker = <Linker<wasmi_wasi::WasiCtx>>::new(&engine);
        Ok(Self { engine, linker })
    }

    pub fn execute(&mut self, wasm_bytes: &[u8], dataset: &[String]) -> Result<(i32, String)> {
        let sandbox_dir = "./swarm_data";
        fs::create_dir_all(sandbox_dir)?;

        // 1. Setup Capability-based WASI Context using the perfectly matched cap-std
        let dir = cap_std::fs::Dir::open_ambient_dir(sandbox_dir, cap_std::ambient_authority())
            .map_err(|e| anyhow!("Failed to open ambient dir: {}", e))?;
            
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .preopened_dir(dir, "/data")
            .map_err(|e| anyhow!("WASI preopened_dir error: {}", e))?
            .build();

        let mut store = Store::new(&self.engine, wasi_ctx);
        store.add_fuel(1_000_000).map_err(|e| anyhow!("Fuel error: {}", e))?;

        // 2. Load Module
        let module = Module::new(&self.engine, wasm_bytes)?;
        
        // 3. Define the Host Memory & WASI Imports for the Guest
        let mut linker = self.linker.clone();
        wasmi_wasi::add_to_linker(&mut linker, |ctx| ctx)
            .map_err(|e| anyhow!("WASI link error: {}", e))?;
        
        let instance = linker.instantiate(&mut store, &module)
            .map_err(|e| anyhow!("Instantiation error: {}", e))?
            .start(&mut store)
            .map_err(|e| anyhow!("Start error: {}", e))?;
        
        // 4. Safe Manual Memory Write
        let joined = dataset.join("\n");
        let payload = joined.as_bytes();
        let ptr_offset = 1_048_576;

        if let Some(memory) = instance.get_memory(&store, "memory") {
            memory.write(&mut store, ptr_offset, payload)
                .map_err(|e| anyhow!("Memory write error: {:?}", e))?;
        }

        // 5. Run Execution
        let execute_func = instance.get_typed_func::<(i32, i32), i32>(&store, "execute")
            .map_err(|e| anyhow!("Export error: {}", e))?;
        
        println!("🔋 Wasmi Interpreter engaged. Executing with virtual stack...");
        
        match execute_func.call(&mut store, (ptr_offset as i32, payload.len() as i32)) {
            Ok(res) => {
                let mut hasher = Sha256::new();
                hasher.update(res.to_le_bytes());
                Ok((res, format!("{:x}", hasher.finalize())))
            }
            Err(e) => Err(anyhow!("Execution Error: {}", e))
        }
    }
}
