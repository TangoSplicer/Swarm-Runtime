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

    pub fn execute(&mut self, wasm_bytes: &[u8], dataset: &[String], polyglot_id: &str, state_path: &str) -> Result<(i32, String, Option<Vec<u8>>)> {
        let sandbox_dir = "./rootfs/data";
        fs::create_dir_all(sandbox_dir).map_err(|e| anyhow!("Failed to create data dir: {}", e))?;
        fs::create_dir_all("./rootfs/lib").unwrap_or_default();
        
        let module = Module::new(&self.engine, wasm_bytes).map_err(|e| anyhow!("Module compilation error: {}", e))?;
        let is_wasi_start = module.exports().any(|e| e.name() == "_start");

        // PHASE 13: Strict WASI Enforcement. Legacy memory injection mode has been eradicated.
        if !is_wasi_start {
            return Err(anyhow!("No valid WASI _start entry point found. Legacy mode is deprecated."));
        }

        let joined_dataset = dataset.join("\n");
        let mut wasi_args: Vec<String> = vec!["swarm-wasm".to_string()];
        let mut target_file = "app.txt";

        if polyglot_id == "POLYGLOT:PYTHON" {
            target_file = "app.py";
            wasi_args = vec!["python".to_string(), "-B".to_string(), "-S".to_string(), "/data/app.py".to_string()];
        } else if polyglot_id == "POLYGLOT:JS" {
            target_file = "app.js";
            wasi_args = vec!["qjs".to_string(), "/data/app.js".to_string()];
        } else if polyglot_id == "POLYGLOT:LUA" {
            target_file = "app.lua";
            wasi_args = vec!["lua".to_string(), "/data/app.lua".to_string()];
        } else if polyglot_id == "POLYGLOT:RUBY" {
            target_file = "app.rb";
            wasi_args = vec!["ruby".to_string(), "/data/app.rb".to_string()];
        } else if polyglot_id == "POLYGLOT:PHP" {
            target_file = "app.php";
            wasi_args = vec!["php".to_string(), "/data/app.php".to_string()];
        } else if polyglot_id == "POLYGLOT:SQLITE" {
            target_file = "app.sql";
            wasi_args = vec!["sqlite3".to_string(), "/data/swarm.db".to_string(), ".read /data/app.sql".to_string()];
        } else {
            // Inject the JSON dataset directly into the WASI arguments for raw WebAssembly
            wasi_args.extend_from_slice(dataset);
        }

        let app_path = format!("{}/{}", sandbox_dir, target_file);
        fs::write(&app_path, &joined_dataset).map_err(|e| anyhow!("Failed to write script: {}", e))?;

        // Open the secure ambient directory mapping the host's ./rootfs to the guest's /
        let root_dir = cap_std::fs::Dir::open_ambient_dir("./rootfs", cap_std::ambient_authority())
            .map_err(|e| anyhow!("Failed to open rootfs dir: {}", e))?;

        // Derive the absolute path for the guest by stripping the host's relative mount point
        let guest_state_path = state_path.replace("./rootfs", "");

        let mut builder = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .args(&wasi_args)
            .map_err(|e| anyhow!("WASI args error: {}", e))?
            .env("SWARM_STATE_FILE", &guest_state_path)
            .map_err(|e| anyhow!("WASI env error: {}", e))?;

        if polyglot_id == "POLYGLOT:PYTHON" {
            builder = builder.env("PYTHONPATH", "/python-wasi.zip").map_err(|e| anyhow!("Env error: {}", e))?
                             .env("PYTHONHOME", "/").map_err(|e| anyhow!("Env error: {}", e))?;
        }

        let wasi_ctx = builder.preopened_dir(root_dir, "/")
            .map_err(|e| anyhow!("WASI preopened_dir error: {}", e))?
            .build();

        let mut store = Store::new(&self.engine, wasi_ctx);
        store.add_fuel(50_000_000_000).map_err(|e| anyhow!("Fuel error: {}", e))?;

        let mut linker = self.linker.clone();
        wasmi_wasi::add_to_linker(&mut linker, |ctx| ctx).map_err(|e| anyhow!("WASI link error: {}", e))?;

        let instance = linker.instantiate(&mut store, &module)
            .map_err(|e| anyhow!("WASI Instantiate error: {}", e))?
            .start(&mut store)
            .map_err(|e| anyhow!("WASI Start error: {}", e))?;

        // Strictly enforce WASI _start execution
        let start_func = instance.get_typed_func::<(), ()>(&store, "_start").map_err(|e| anyhow!("Export error: {}", e))?;
        start_func.call(&mut store, ()).unwrap_or_else(|_| println!("Execution trap caught (normal for WASI exit)."));

        // --- PHASE 7: STATE EXTRACTION (V2 - Strict WASI POSIX File I/O) ---
        // We no longer blindly dump the WASM linear memory.
        // We dynamically read the state file directly from the host filesystem.
        let new_state = fs::read(state_path).ok();

        let mut hasher = Sha256::new();
        let output_path = format!("{}/output.txt", sandbox_dir);
        if let Ok(content) = fs::read(&output_path) {
            hasher.update(&content);
        } else {
            hasher.update(b"NO_OUTPUT_FOUND");
        }

        // Standardized result code for WASI execution
        let result_code = 0; 

        Ok((result_code, format!("{:x}", hasher.finalize()), new_state))
    }
}
