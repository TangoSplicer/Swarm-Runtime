import os

# --- 1. OVERWRITE JUDGE TO BE LANGUAGE AGNOSTIC ---
judge_code = """use anyhow::{anyhow, Result};
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

    pub fn execute(&mut self, wasm_bytes: &[u8], dataset: &[String], polyglot_id: &str) -> Result<(i32, String)> {
        let sandbox_dir = "./rootfs/data";
        fs::create_dir_all(sandbox_dir).map_err(|e| anyhow!("Failed to create data dir: {}", e))?;
        fs::create_dir_all("./rootfs/lib").unwrap_or_default();

        let module = Module::new(&self.engine, wasm_bytes).map_err(|e| anyhow!("Module compilation error: {}", e))?;
        let is_wasi_start = module.exports().any(|e| e.name() == "_start");
        let is_legacy = module.exports().any(|e| e.name() == "execute");

        if !is_wasi_start && !is_legacy {
            return Err(anyhow!("No valid entry point found."));
        }

        let joined_dataset = dataset.join("\\n");
        let mut wasi_args: Vec<String> = vec!["swarm-wasm".to_string()]; 
        let mut target_file = "app.txt";

        // DYNAMIC POLYGLOT ROUTING
        if is_wasi_start {
            if polyglot_id == "POLYGLOT:PYTHON" {
                target_file = "app.py";
                wasi_args = vec!["python".to_string(), "-B".to_string(), "-S".to_string(), "/data/app.py".to_string()];
            } else if polyglot_id == "POLYGLOT:JS" {
                target_file = "app.js";
                wasi_args = vec!["qjs".to_string(), "/data/app.js".to_string()];
            }
            
            let app_path = format!("{}/{}", sandbox_dir, target_file);
            fs::write(&app_path, &joined_dataset).map_err(|e| anyhow!("Failed to write script: {}", e))?;
        }

        let root_dir = cap_std::fs::Dir::open_ambient_dir("./rootfs", cap_std::ambient_authority())
            .map_err(|e| anyhow!("Failed to open rootfs dir: {}", e))?;
            
        let mut builder = WasiCtxBuilder::new();
        builder.inherit_stdout().inherit_stderr().args(&wasi_args).unwrap();
        
        if polyglot_id == "POLYGLOT:PYTHON" {
            builder.env("PYTHONPATH", "/python-wasi.zip").unwrap().env("PYTHONHOME", "/").unwrap();
        }

        let wasi_ctx = builder.preopened_dir(root_dir, "/").unwrap().build();

        let mut store = Store::new(&self.engine, wasi_ctx);
        store.add_fuel(50_000_000_000).unwrap();

        let mut linker = self.linker.clone();
        wasmi_wasi::add_to_linker(&mut linker, |ctx| ctx).unwrap();
        
        let instance = linker.instantiate(&mut store, &module).unwrap().start(&mut store).unwrap();
        let mut result_code = 0;

        if is_legacy {
            let payload = joined_dataset.as_bytes();
            let ptr_offset = 1_048_576;
            if let Some(memory) = instance.get_memory(&store, "memory") {
                memory.write(&mut store, ptr_offset, payload).unwrap();
            }
            let execute_func = instance.get_typed_func::<(i32, i32), i32>(&store, "execute").unwrap();
            result_code = execute_func.call(&mut store, (ptr_offset as i32, payload.len() as i32)).unwrap();
        } else {
            let start_func = instance.get_typed_func::<(), ()>(&store, "_start").unwrap(); 
            start_func.call(&mut store, ()).unwrap_or_else(|_| println!("Execution trap caught (normal for WASI exit)."));
        }

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

        Ok((result_code, format!("{:x}", hasher.finalize())))
    }
}
"""
with open("components/judge/src/lib.rs", "w") as f:
    f.write(judge_code)

# --- 2. PATCH WORKER TO LOAD QJS.WASM ---
worker_path = "components/swarm-node/src/worker.rs"
with open(worker_path, "r") as f:
    worker_code = f.read()

# Replace the old binary loader with the new Polyglot router
old_loader = r'if let Ok(wasm) = if shard_data.wasm_image == "POLYGLOT:PYTHON" { Ok(std::fs::read("python.wasm").unwrap_or_default()) } else { general_purpose::STANDARD.decode(&shard_data.wasm_image) } {'
new_loader = """let polyglot_id = if shard_data.wasm_image.starts_with("POLYGLOT:") { shard_data.wasm_image.clone() } else { "NONE".to_string() };
                                                let wasm_result = match polyglot_id.as_str() {
                                                    "POLYGLOT:PYTHON" => Ok(std::fs::read("python.wasm").unwrap_or_default()),
                                                    "POLYGLOT:JS" => Ok(std::fs::read("qjs.wasm").unwrap_or_default()),
                                                    _ => general_purpose::STANDARD.decode(&shard_data.wasm_image),
                                                };
                                                if let Ok(wasm) = wasm_result {"""
worker_code = worker_code.replace(old_loader, new_loader)

# Update Judge execution call to pass the polyglot ID
worker_code = worker_code.replace('match judge.execute(&wasm, &shard_data.dataset) {', 'match judge.execute(&wasm, &shard_data.dataset, &polyglot_id) {')
with open(worker_path, "w") as f:
    f.write(worker_code)

# --- 3. PATCH CLI TO SUPPORT JS ---
cli_path = "components/swarm-cli/src/main.rs"
with open(cli_path, "r") as f:
    cli_code = f.read()

old_cli_match = """                "python" => "POLYGLOT:PYTHON",
                _ => anyhow::bail!("Unsupported language: {}. Currently supported: python", lang),"""
new_cli_match = """                "python" => "POLYGLOT:PYTHON",
                "js" | "javascript" => "POLYGLOT:JS",
                _ => anyhow::bail!("Unsupported language: {}. Currently supported: python, js", lang),"""
cli_code = cli_code.replace(old_cli_match, new_cli_match)
with open(cli_path, "w") as f:
    f.write(cli_code)

print("✅ Universal Polyglot Architecture injected!")
