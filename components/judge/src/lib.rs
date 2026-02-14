use anyhow::{Result, anyhow};
use wasmer::{Instance, Module, Store, imports, Value, Function, FunctionEnv, FunctionEnvMut};
use std::sync::Arc;

/// The context injected into the Wasm environment.
/// Holds the database connection and the specific shard range for this task.
#[derive(Clone)]
pub struct JudgeContext {
    db: Option<Arc<sled::Db>>,
    shard_start: u64,
    shard_end: u64,
}

pub struct Judge {
    store: Store,
    db: Option<Arc<sled::Db>>,
}

impl Judge {
    pub fn new(db: Option<Arc<sled::Db>>) -> Result<Self> {
        Ok(Self {
            store: Store::default(),
            db,
        })
    }

    /// Executes the Wasm module with an optional Shard Range.
    /// If range is None, defaults to (0, 0).
    pub fn execute(&mut self, wasm_bytes: &[u8], range: Option<(u64, u64)>) -> Result<i32> {
        let module = Module::new(&self.store, wasm_bytes)?;

        let (start, end) = range.unwrap_or((0, 0));

        // Create the environment with our new Context struct
        let ctx = JudgeContext {
            db: self.db.clone(),
            shard_start: start,
            shard_end: end,
        };

        let env = FunctionEnv::new(&mut self.store, ctx);

        // --- HOST FUNCTION: db_set ---
        let db_set = Function::new_typed_with_env(&mut self.store, &env, |env: FunctionEnvMut<JudgeContext>, key: i32, val: i32| {
            let ctx = env.data();
            if let Some(db) = &ctx.db {
                let k = key.to_be_bytes();
                let v = val.to_be_bytes();
                let _ = db.insert(&k, &v);
                // In production, you might not want to flush every write, but good for debugging
                let _ = db.flush(); 
                println!("Judge: DB_SET [{}, {}] -> {} = {}", ctx.shard_start, ctx.shard_end, key, val);
            }
        });

        // --- HOST FUNCTION: db_get ---
        let db_get = Function::new_typed_with_env(&mut self.store, &env, |env: FunctionEnvMut<JudgeContext>, key: i32| -> i32 {
            let ctx = env.data();
            if let Some(db) = &ctx.db {
                let k = key.to_be_bytes();
                if let Ok(Some(ivec)) = db.get(&k) {
                    if ivec.len() >= 4 {
                        let mut bytes = [0u8; 4];
                        bytes.copy_from_slice(&ivec[0..4]);
                        return i32::from_be_bytes(bytes);
                    }
                }
            }
            -1
        });

        // --- HOST FUNCTION: get_shard_start ---
        let get_shard_start = Function::new_typed_with_env(&mut self.store, &env, |env: FunctionEnvMut<JudgeContext>| -> i64 {
            env.data().shard_start as i64
        });

        // --- HOST FUNCTION: get_shard_end ---
        let get_shard_end = Function::new_typed_with_env(&mut self.store, &env, |env: FunctionEnvMut<JudgeContext>| -> i64 {
            env.data().shard_end as i64
        });

        let import_object = imports! {
            "env" => {
                "db_set" => db_set,
                "db_get" => db_get,
                "get_shard_start" => get_shard_start,
                "get_shard_end" => get_shard_end,
            }
        };

        let instance = Instance::new(&mut self.store, &module, &import_object)?;
        let entry_point = instance.exports.get_function("main")
            .or_else(|_| instance.exports.get_function("_start"))?;

        let params = vec![Value::I32(0); entry_point.ty(&self.store).params().len()];
        let result = entry_point.call(&mut self.store, &params)?;

        if let Some(val) = result.first() {
            match val {
                Value::I32(i) => Ok(*i),
                _ => Err(anyhow!("Unexpected return type")),
            }
        } else {
            Ok(0)
        }
    }
}
