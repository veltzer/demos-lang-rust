use std::io;
use std::process;

fn main() {
    let my_pid = process::id();
    println!("Please monitor my memory via [top -p {}]...", my_pid);
    println!("Starting, press any key...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Create a large vector
    let v: Vec<i32> = (0..1000*my_pid as i32).collect();
    println!("Vector length: {}", v.len());
    println!("Vector created, press any key...");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Explicitly delete the vector
    drop(v);
    println!("Vector has been deleted, press any key...");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Try to access the deleted vector
    // This will result in a compile-time or runtime error
    // println!("Vector length: {}", v.len());
}
