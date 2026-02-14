// Define the interface provided by the Swarm-Runtime Judge
extern "C" {
    fn db_set(key: i32, val: i32);
    fn db_get(key: i32) -> i32;
    fn get_shard_start() -> i64;
    fn get_shard_end() -> i64;
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    unsafe {
        // 1. Ask the Host (Judge) for our assigned range
        let start = get_shard_start();
        let end = get_shard_end();

        // 2. Perform the "Computation"
        // We will sum the numbers in this range.
        // Example: If range is 0..10, sum = 0+1+2...+9
        let mut sum: i64 = 0;
        for i in start..end {
            sum += i;
        }

        // 3. Store the result
        // Since db_set takes i32, we'll cast it (assuming small test numbers)
        // We use the start of the range as the key ID, so different shards don't overwrite each other
        db_set(start as i32, sum as i32);
        
        // 4. Also store a "status flag" at key 100 + shard_index just to prove we ran
        // (Simulating a simple way to verify execution)
        db_set(100 + (start as i32), 1);

        return sum as i32;
    }
}
