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

    // NEW: We now accept the raw array slice directly
    pub fn execute(&mut self, wasm_bytes: &[u8], data: &[i32]) -> Result<i32> {
        let module = Module::new(&self.store, wasm_bytes)?;
        let import_object = Imports::new();
        let instance = Instance::new(&mut self.store, &module, &import_object)?;
        self.instance = Some(instance.clone());

        // MEMORY INJECTION
        if let Ok(memory) = instance.exports.get_memory("memory") {
            let view = memory.view(&self.store);
            
            // Convert &[i32] to &[u8] bytes for Wasm
            let mut byte_data = Vec::with_capacity(data.len() * 4);
            for &num in data {
                byte_data.extend_from_slice(&num.to_le_bytes());
            }
            
            // Write dataset into Wasm sandbox starting at memory offset 0
            view.write(0, &byte_data).map_err(|_| anyhow!("Judge: Failed to write to Wasm memory"))?;
        } else {
            println!("⚠️ Judge: Wasm module does not export 'memory'. Just passing length.");
        }

        let execute_func = instance.exports.iter().find_map(|(_, export)| {
            if let Extern::Function(f) = export { Some(f.clone()) } else { None }
        }).ok_or_else(|| anyhow!("Judge: No exported function found"))?;

        // STRICT SIGNATURE CHECK: (pointer: i32, length: i32) -> i32
        let params = execute_func.ty(&self.store).params().to_vec();
        let results = execute_func.ty(&self.store).results().to_vec();
        if params != vec![Type::I32, Type::I32] || results != vec![Type::I32] {
            return Err(anyhow!("Judge: Signature mismatch. Expected (i32, i32) -> i32."));
        }

        // Pass Pointer (0) and Length (data.len())
        let result_values = execute_func.call(&mut self.store, &[
            wasmer::Value::I32(0),
            wasmer::Value::I32(data.len() as i32)
        ])?;
        
        let first_val = result_values.get(0).ok_or_else(|| anyhow!("Judge: Wasm returned nothing"))?;
        
        if let wasmer::Value::I32(ret) = first_val {
            Ok(*ret)
        } else {
            Err(anyhow!("Judge: Wasm execution returned non-i32"))
        }
    }
}
