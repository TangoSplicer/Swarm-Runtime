// A global counter that lives in the Wasm linear memory.
static mut COUNTER: i32 = 0;

#[no_mangle]
pub extern "C" fn execute(_ptr: i32, _len: i32) -> i32 {
    unsafe {
        // Increment the stateful counter
        COUNTER += 1;
        
        // Return the new counter value to the Judge
        COUNTER
    }
}
