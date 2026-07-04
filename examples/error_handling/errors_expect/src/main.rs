/*
 * This example shows the .expect() method of the Result type of Rust.
 *
 * .expect(msg) behaves like .unwrap() but lets you supply your own panic
 * message. If the Result is Ok(v) it returns v, and if it is Err(e) it
 * panics with your message followed by the error.
 *
 * Prefer .expect() over .unwrap() when you want a clear explanation of why
 * you believe the operation cannot fail (or what went wrong if it does).
 */
use std::fs::File;

fn main() {
    // This file does not exist, so .expect() will panic and print our
    // message together with the underlying error.
    let _greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    println!("if you see this, the file was opened successfully");
}
