use anyhow::{Result, anyhow};
use wasmer::{Instance, Module, Store, imports, Value};

pub struct Judge {
    store: Store,
}

impl Judge {
    pub fn new() -> Result<Self> {
        Ok(Self {
            store: Store::default(),
        })
    }

    pub fn execute(&mut self, wasm_bytes: &[u8]) -> Result<i32> {
        let module = Module::new(&self.store, wasm_bytes)?;
        let import_object = imports! {};
        let instance = Instance::new(&mut self.store, &module, &import_object)?;

        // Try to find 'main' or '_start' (common Wasm entry points)
        let entry_point = instance.exports.get_function("main")
            .or_else(|_| instance.exports.get_function("_start"))
            .map_err(|_| anyhow!("Could not find an entry point (main or _start) in Wasm module"))?;

        // Call the function - we allow it to have arguments but pass none
        // This bypasses the strict signature check we hit earlier
        let params = vec![Value::I32(0); entry_point.ty(&self.store).params().len()];
        let result = entry_point.call(&mut self.store, &params)?;

        if let Some(val) = result.first() {
            match val {
                Value::I32(i) => Ok(*i),
                _ => Err(anyhow!("Unexpected return type from Wasm")),
            }
        } else {
            Ok(0)
        }
    }
}
