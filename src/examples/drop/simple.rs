use std::io;
use std::process;

fn main() {
    let my_pid = process::id();
    println!("Please monitor my memory via [top -p {}]...", my_pid);
    println!("Starting, press any key...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Create a large vector
    let mut v: Vec<i32> = (0..1000*my_pid as i32).collect();
    //let mut v: Vec<i32> = Vec::with_capacity(10000000);
    println!("Vector created with capacity");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // resizing the vector
    v.resize(10000, 1);
    println!("Vector resized");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // pushing some values
    for i in 0..100000 {
        v.push(i);
    }
    println!("pushed some values...");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Explicitly delete the vector
    drop(v);
    println!("Vector has been deleted, press any key...");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Try to access the deleted vector
    // This will result in a compile-time or runtime error
    // println!("Vector length: {}", v.len());
}
