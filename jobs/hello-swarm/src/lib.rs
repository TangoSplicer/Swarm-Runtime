#[no_mangle]
pub extern "C" fn main() -> i32 {
    let mut x = 0;
    for i in 0..100 {
        x += i;
    }
    x 
}
