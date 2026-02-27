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
        fs::create_dir_all(sandbox_dir)
            .map_err(|e| anyhow!("Failed to create sandbox dir: {}", e))?;

        let module = Module::new(&self.engine, wasm_bytes)
            .map_err(|e| anyhow!("Module compilation error: {}", e))?;

        // 1. Inspect the WebAssembly Module Exports
        let is_wasi_start = module.exports().any(|e| e.name() == "_start");
        let is_legacy = module.exports().any(|e| e.name() == "execute");

        if !is_wasi_start && !is_legacy {
            return Err(anyhow!("No valid entry point found. Expected '_start' or 'execute'."));
        }

        let joined_dataset = dataset.join("\n");
        let mut wasi_args: Vec<String> = vec!["swarm-wasm".to_string()]; 

        if is_wasi_start {
            // 2. Write the dataset directly into the sandboxed VMFS
            let app_py_path = format!("{}/app.py", sandbox_dir);
            fs::write(&app_py_path, &joined_dataset)
                .map_err(|e| anyhow!("Failed to write app.py to VMFS: {}", e))?;
            
            // 3. Configure WASI Context arguments for the polyglot interpreter
            wasi_args = vec!["python".to_string(), "/data/app.py".to_string()];
        }

        // Setup Capability-based WASI Context using cap-std ambient authority
        let dir = cap_std::fs::Dir::open_ambient_dir(sandbox_dir, cap_std::ambient_authority())
            .map_err(|e| anyhow!("Failed to open ambient dir: {}", e))?;
            
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .args(&wasi_args)
            .map_err(|e| anyhow!("WASI args error: {}", e))?
            .preopened_dir(dir, "/data")
            .map_err(|e| anyhow!("WASI preopened_dir error: {}", e))?
            .build();

        let mut store = Store::new(&self.engine, wasi_ctx);
        store.set_fuel(5_000_000)
            .map_err(|e| anyhow!("Failed to set fuel limit: {}", e))?;

        // Define the Host Memory & WASI Imports for the Guest
        let mut linker = self.linker.clone();
        wasmi_wasi::add_to_linker(&mut linker, |ctx| ctx)
            .map_err(|e| anyhow!("WASI link error: {}", e))?;
        
        let instance = linker.instantiate(&mut store, &module)
            .map_err(|e| anyhow!("Instantiation error: {}", e))?
            .start(&mut store)
            .map_err(|e| anyhow!("Start error: {}", e))?;
        
        let mut result_code = 0;

        if is_legacy {
            // 4A. Legacy Execution: Safe Manual Memory Write to 1MB Offset
            let payload = joined_dataset.as_bytes();
            let ptr_offset = 1_048_576;

            if let Some(memory) = instance.get_memory(&store, "memory") {
                memory.write(&mut store, ptr_offset, payload)
                    .map_err(|e| anyhow!("Memory write error: {}", e))?;
            }

            let execute_func = instance.get_typed_func::<(i32, i32), i32>(&store, "execute")
                .map_err(|e| anyhow!("Export error: {}", e))?;
            
            println!("🔋 Wasmi Interpreter engaged. Executing legacy Rust payload...");
            
            result_code = execute_func.call(&mut store, (ptr_offset as i32, payload.len() as i32))
                .map_err(|e| anyhow!("Legacy execution trap: {}", e))?;

        } else if is_wasi_start {
            // 4B. Polyglot Execution: Standard WASI _start
            let start_func = instance.get_typed_func::<(), ()>(&store, "_start")
                .map_err(|e| anyhow!("Export error: {}", e))?;
            
            println!("🐍 Wasmi Interpreter engaged. Executing polyglot _start...");
            
            start_func.call(&mut store, ())
                .map_err(|e| anyhow!("Polyglot execution trap: {}", e))?;
        }

        // 5. Output State Consensus / VMFS Sweeper
        let mut hasher = Sha256::new();
        
        if is_wasi_start {
            let output_path = format!("{}/output.txt", sandbox_dir);
            if let Ok(content) = fs::read(&output_path) {
                hasher.update(&content);
            } else {
                hasher.update(b"NO_OUTPUT_FOUND");
            }
        } else {
            hasher.update(result_code.to_le_bytes());
        }

        let hash_result = format!("{:x}", hasher.finalize());
        
        Ok((result_code, hash_result))
    }
}
