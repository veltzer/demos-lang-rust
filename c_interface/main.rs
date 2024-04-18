use std::os::raw::c_int;

// Define the C function in Rust
#[link(name = "add")]
extern "C" {
    fn add(a: c_int, b: c_int) -> c_int;
}

fn main() {
    let result = unsafe { add(5, 7) };
    println!("5 + 7 = {}", result);
}
