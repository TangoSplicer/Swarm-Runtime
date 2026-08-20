use anyhow::{anyhow, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use wasmi::*;
use wasmi_wasi::WasiCtxBuilder;

const DEFAULT_FUEL_LIMIT: u64 = 50_000_000;
const MAX_MODULE_BYTES: usize = 25 * 1024 * 1024;
const MAX_DATASET_BYTES: usize = 5 * 1024 * 1024;
const MAX_STATE_BYTES: u64 = 5 * 1024 * 1024;
const MAX_OUTPUT_BYTES: u64 = 5 * 1024 * 1024;

pub struct Judge {
    engine: Engine,
    linker: Linker<wasmi_wasi::WasiCtx>,
    fuel_limit: u64,
}

impl Judge {
    pub fn new(gas_limit: Option<u64>) -> Result<Self> {
        let fuel_limit = gas_limit.unwrap_or(DEFAULT_FUEL_LIMIT);
        if fuel_limit == 0 {
            return Err(anyhow!("Execution fuel limit must be greater than zero"));
        }

        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);
        let linker = <Linker<wasmi_wasi::WasiCtx>>::new(&engine);
        Ok(Self {
            engine,
            linker,
            fuel_limit,
        })
    }

    pub fn execute(
        &mut self,
        wasm_bytes: &[u8],
        dataset: &[String],
        polyglot_id: &str,
        workspace_dir: &Path,
    ) -> Result<(i32, String, Option<Vec<u8>>)> {
        if wasm_bytes.len() > MAX_MODULE_BYTES {
            return Err(anyhow!(
                "WebAssembly module exceeds the {} byte execution limit",
                MAX_MODULE_BYTES
            ));
        }

        let dataset_size = dataset_size(dataset)?;
        if dataset_size > MAX_DATASET_BYTES {
            return Err(anyhow!(
                "Dataset exceeds the {} byte execution limit",
                MAX_DATASET_BYTES
            ));
        }

        let data_dir = workspace_dir.join("data");
        fs::create_dir_all(&data_dir)
            .map_err(|e| anyhow!("Failed to create isolated workspace: {e}"))?;

        let module = Module::new(&self.engine, wasm_bytes)
            .map_err(|e| anyhow!("Module compilation error: {e}"))?;
        if !module.exports().any(|export| export.name() == "_start") {
            return Err(anyhow!(
                "No valid WASI _start entry point found. Legacy mode is deprecated."
            ));
        }

        let joined_dataset = dataset.join("\n");
        let (target_file, wasi_args) = wasi_invocation(polyglot_id, dataset);
        let app_path = data_dir.join(target_file);
        fs::write(&app_path, joined_dataset)
            .map_err(|e| anyhow!("Failed to write isolated script input: {e}"))?;

        let root_dir =
            cap_std::fs::Dir::open_ambient_dir(workspace_dir, cap_std::ambient_authority())
                .map_err(|e| anyhow!("Failed to open isolated workspace: {e}"))?;
        let guest_state_path = "/data/state.json";

        let mut builder = WasiCtxBuilder::new();
        builder
            .inherit_stdout()
            .inherit_stderr()
            .args(&wasi_args)
            .map_err(|e| anyhow!("WASI args error: {e}"))?
            .env("SWARM_STATE_FILE", guest_state_path)
            .map_err(|e| anyhow!("WASI env error: {e}"))?;

        if polyglot_id == "POLYGLOT:PYTHON" {
            builder
                .env("PYTHONPATH", "/python-wasi.zip")
                .map_err(|e| anyhow!("WASI environment error: {e}"))?
                .env("PYTHONHOME", "/")
                .map_err(|e| anyhow!("WASI environment error: {e}"))?;
        }

        let wasi_ctx = builder
            .preopened_dir(root_dir, "/")
            .map_err(|e| anyhow!("WASI preopened_dir error: {e}"))?
            .build();

        let mut store = Store::new(&self.engine, wasi_ctx);
        store
            .set_fuel(self.fuel_limit)
            .map_err(|e| anyhow!("Fuel error: {e}"))?;

        let mut linker = self.linker.clone();
        wasmi_wasi::add_to_linker(&mut linker, |ctx| ctx)
            .map_err(|e| anyhow!("WASI link error: {e}"))?;

        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .map_err(|e| anyhow!("WASI start error: {e}"))?;

        let start_func = instance
            .get_typed_func::<(), ()>(&store, "_start")
            .map_err(|e| anyhow!("WASI _start export error: {e}"))?;
        start_func
            .call(&mut store, ())
            .map_err(|e| anyhow!("WASI execution trapped: {e}"))?;

        let state_path = workspace_state_path(workspace_dir);
        if let Ok(metadata) = fs::metadata(&state_path) {
            if metadata.len() > MAX_STATE_BYTES {
                return Err(anyhow!(
                    "Execution state exceeds the {} byte limit",
                    MAX_STATE_BYTES
                ));
            }
        }
        let new_state = fs::read(&state_path).ok();
        let output_path = data_dir.join("output.txt");
        if let Ok(metadata) = fs::metadata(&output_path) {
            if metadata.len() > MAX_OUTPUT_BYTES {
                return Err(anyhow!(
                    "Execution output exceeds the {} byte limit",
                    MAX_OUTPUT_BYTES
                ));
            }
        }

        let mut hasher = Sha256::new();
        if let Ok(content) = fs::read(&output_path) {
            hasher.update(&content);
        } else {
            hasher.update(b"NO_OUTPUT_FOUND");
        }

        Ok((0, format!("{:x}", hasher.finalize()), new_state))
    }
}

pub fn workspace_state_path(workspace_dir: &Path) -> PathBuf {
    workspace_dir.join("data").join("state.json")
}

fn dataset_size(dataset: &[String]) -> Result<usize> {
    dataset.iter().try_fold(0_usize, |size, item| {
        size.checked_add(item.len())
            .and_then(|size| size.checked_add(1))
            .ok_or_else(|| anyhow!("Dataset size overflow"))
    })
}

fn wasi_invocation(polyglot_id: &str, dataset: &[String]) -> (&'static str, Vec<String>) {
    match polyglot_id {
        "POLYGLOT:PYTHON" => (
            "app.py",
            vec![
                "python".to_string(),
                "-B".to_string(),
                "-S".to_string(),
                "/data/app.py".to_string(),
            ],
        ),
        "POLYGLOT:JS" => (
            "app.js",
            vec!["qjs".to_string(), "/data/app.js".to_string()],
        ),
        "POLYGLOT:LUA" => (
            "app.lua",
            vec!["lua".to_string(), "/data/app.lua".to_string()],
        ),
        "POLYGLOT:RUBY" => (
            "app.rb",
            vec!["ruby".to_string(), "/data/app.rb".to_string()],
        ),
        "POLYGLOT:PHP" => (
            "app.php",
            vec!["php".to_string(), "/data/app.php".to_string()],
        ),
        "POLYGLOT:SQLITE" => (
            "app.sql",
            vec![
                "sqlite3".to_string(),
                "/data/swarm.db".to_string(),
                ".read /data/app.sql".to_string(),
            ],
        ),
        _ => {
            let mut args = vec!["swarm-wasm".to_string()];
            args.extend_from_slice(dataset);
            ("app.txt", args)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_fuel_limit_is_bounded() {
        let judge = Judge::new(None).expect("the default fuel limit must be valid");
        assert_eq!(judge.fuel_limit, DEFAULT_FUEL_LIMIT);
        assert!(judge.fuel_limit < 50_000_000_000);
    }

    #[test]
    fn zero_fuel_limit_is_rejected() {
        assert!(Judge::new(Some(0)).is_err());
    }

    #[test]
    fn dataset_limit_accounts_for_all_items() {
        assert_eq!(
            dataset_size(&["abc".to_string(), "de".to_string()]).unwrap(),
            7
        );
        assert!(dataset_size(&["x".repeat(MAX_DATASET_BYTES)]).unwrap() > MAX_DATASET_BYTES);
    }

    #[test]
    fn state_path_is_scoped_to_workspace() {
        let workspace = Path::new("/tmp/swarm-workspaces/job-a");
        assert_eq!(
            workspace_state_path(workspace),
            Path::new("/tmp/swarm-workspaces/job-a/data/state.json")
        );
    }

    #[test]
    fn stable_runtime_executes_minimal_start_module_in_isolated_workspace() {
        let workspace = std::env::temp_dir().join(format!(
            "swarm-judge-runtime-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock must be after the Unix epoch")
                .as_nanos()
        ));
        let mut judge = Judge::new(Some(10_000)).expect("fuel limit must be valid");

        let result = judge.execute(
            &minimal_start_module(),
            &["input line".to_string()],
            "WASM",
            &workspace,
        );
        let app_contents = std::fs::read_to_string(workspace.join("data/app.txt"));
        let cleanup = std::fs::remove_dir_all(&workspace);

        assert!(result.is_ok(), "minimal module must execute: {result:?}");
        assert_eq!(result.expect("result checked above").0, 0);
        assert_eq!(
            app_contents.expect("input file must be written"),
            "input line"
        );
        assert!(cleanup.is_ok(), "test workspace cleanup must succeed");
    }

    fn minimal_start_module() -> Vec<u8> {
        vec![
            0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00, // Wasm header
            0x01, 0x04, 0x01, 0x60, 0x00, 0x00, // () -> () function type
            0x03, 0x02, 0x01, 0x00, // one function using type zero
            0x07, 0x0A, 0x01, 0x06, b'_', b's', b't', b'a', b'r', b't', 0x00, 0x00, 0x0A, 0x04,
            0x01, 0x02, 0x00, 0x0B, // empty function body
        ]
    }
}
