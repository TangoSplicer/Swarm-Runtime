use anyhow::Result;
use wasmer::{Store, Module, Instance, Value, Imports, TypedFunction};

pub struct Judge {
    store: Store,
}

impl Judge {
    pub fn new(_db: Option<sled::Db>) -> Result<Self> {
        Ok(Self { store: Store::default() })
    }

    pub fn execute(&mut self, wasm_bytes: &[u8], range: Option<(u64, u64)>) -> Result<i32> {
        let module = Module::new(&self.store, wasm_bytes)?;
        let import_object = Imports::new();
        let instance = Instance::new(&mut self.store, &module, &import_object)?;

        // STRICTLY LOOK FOR (i32, i32) -> i32
        let execute_func: TypedFunction<(i32, i32), i32> = instance.exports.get_typed_function(&self.store, "execute")?;

        let (start, end) = range.unwrap_or((0, 0));
        
        // CAST u64 -> i32 (This fixes the mismatch!)
        let start_32 = start as i32;
        let end_32 = end as i32;

        let result = execute_func.call(&mut self.store, start_32, end_32)?;
        Ok(result)
    }
}
