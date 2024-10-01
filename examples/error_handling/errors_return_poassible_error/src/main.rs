/*
 * This example shows how to write a function which returns a value or an error.
 */
use std::io;

fn divide_by_two(num: i32) -> Option<i32> {
    if num % 2 == 0 {
        Some(num/2)
    } else {
        None
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let x: i32 = input_line.trim().parse().unwrap();
    // you must call unwrap
    let result = divide_by_two(x).unwrap();
    println!("divided by 2 is {result}")
}
