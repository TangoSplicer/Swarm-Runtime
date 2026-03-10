use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut sum = 0;

    for arg in args {
        if let Ok(num) = arg.parse::<i32>() {
            sum += num;
        }
    }

    let result_string = format!("GLOBAL MESH SUCCESS! The sum is: {}", sum);
    
    // Print to stdout
    println!("{}", result_string);
    
    // FIX: Write exactly to the mounted /data folder!
    let _ = fs::write("/data/output.txt", result_string);
}
