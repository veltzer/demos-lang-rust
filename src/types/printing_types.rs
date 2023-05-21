// this example shows you how to get and print types of variables in rust
//
// References:
// - https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable

use std::any::type_name;

fn get_type_name<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}

fn main() {
    let x: i32=7;
    print_type_of(&x);
    println!("type of x is {}", get_type_name(&x));
}
