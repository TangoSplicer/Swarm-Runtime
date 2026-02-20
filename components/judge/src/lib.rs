use anyhow::{Result, anyhow};
use wasmer::{Store, Module, Instance, Imports, Type, Extern};

pub struct Judge {
    store: Store,
    instance: Option<Instance>,
}

impl Judge {
    pub fn new(_wasm_bytes: Option<&[u8]>) -> Result<Self> {
        let store = Store::default();
        Ok(Self { store, instance: None })
    }

    pub fn execute(&mut self, wasm_bytes: &[u8], range: Option<(u64, u64)>) -> Result<i32> {
        let module = Module::new(&self.store, wasm_bytes)?;
        let import_object = Imports::new();
        let instance = Instance::new(&mut self.store, &module, &import_object)?;
        self.instance = Some(instance.clone());

        // ROBUSTNESS: Safely match the first exported function
        let execute_func = instance.exports.iter().find_map(|(_, export)| {
            if let Extern::Function(f) = export {
                Some(f.clone())
            } else {
                None
            }
        }).ok_or_else(|| anyhow!("Judge: No exported function found in Wasm"))?;

        let params = execute_func.ty(&self.store).params().to_vec();
        let results = execute_func.ty(&self.store).results().to_vec();

        if params != vec![Type::I32, Type::I32] || results != vec![Type::I32] {
            return Err(anyhow!("Judge: Wasm signature mismatch. Expected (i32, i32) -> i32."));
        }

        let start = range.map(|r| r.0 as i32).unwrap_or(0);
        let end = range.map(|r| r.1 as i32).unwrap_or(0);

        let result_values = execute_func.call(&mut self.store, &[wasmer::Value::I32(start), wasmer::Value::I32(end)])?;
        
        let first_val = result_values.get(0).ok_or_else(|| anyhow!("Judge: Wasm returned nothing"))?;
        
        if let wasmer::Value::I32(ret) = first_val {
            Ok(*ret)
        } else {
            Err(anyhow!("Judge: Wasm execution returned non-i32"))
        }
    }
}
