use anyhow::{Result, anyhow};
use wasmer::{Instance, Module, Store, imports, Value, Function, FunctionEnv, FunctionEnvMut};
use std::sync::Arc;

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

    pub fn execute(&mut self, wasm_bytes: &[u8]) -> Result<i32> {
        let module = Module::new(&self.store, wasm_bytes)?;
        
        let db_handle = self.db.clone();
        let env = FunctionEnv::new(&mut self.store, db_handle);

        // --- HOST FUNCTION: db_set ---
        let db_set = Function::new_typed_with_env(&mut self.store, &env, |mut _env: FunctionEnvMut<Option<Arc<sled::Db>>>, key: i32, val: i32| {
            if let Some(db) = _env.data() {
                let k = key.to_be_bytes();
                let v = val.to_be_bytes();
                let _ = db.insert(&k, &v);
                let _ = db.flush();
                println!("Judge (Host): DB_SET -> {} = {}", key, val);
            }
        });

        // --- HOST FUNCTION: db_get ---
        let db_get = Function::new_typed_with_env(&mut self.store, &env, |mut _env: FunctionEnvMut<Option<Arc<sled::Db>>>, key: i32| -> i32 {
            if let Some(db) = _env.data() {
                let k = key.to_be_bytes();
                if let Ok(Some(ivec)) = db.get(&k) {
                    if ivec.len() >= 4 {
                        let mut bytes = [0u8; 4];
                        bytes.copy_from_slice(&ivec[0..4]);
                        let val = i32::from_be_bytes(bytes);
                        println!("Judge (Host): DB_GET -> {} = {}", key, val);
                        return val;
                    }
                }
            }
            -1 // Return -1 if not found
        });

        let import_object = imports! {
            "env" => {
                "db_set" => db_set,
                "db_get" => db_get,
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
