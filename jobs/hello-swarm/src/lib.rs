extern "C" {
    fn db_set(key: i32, val: i32);
    fn db_get(key: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let key = 101;
    
    // 1. Get current count
    let mut current_count = unsafe { db_get(key) };
    
    // 2. If it's the first time (-1), start at 0
    if current_count == -1 {
        current_count = 0;
    }
    
    // 3. Increment
    let new_count = current_count + 1;
    
    // 4. Save back to the Swarm
    unsafe {
        db_set(key, new_count);
    }
    
    new_count
}
