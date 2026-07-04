/*
 * This example shows how to propagate ("wrap") errors upward using the
 * ? operator instead of handling them on the spot.
 *
 * When you call a fallible function and follow it with ?:
 * - if the result is Ok(v), the value v is unwrapped and evaluation continues
 * - if the result is Err(e), the error is returned from the enclosing
 *   function immediately (after converting it into the function's error type)
 *
 * Here read_username reads a file and propagates any io::Error to its caller.
 * main also returns a Result, so it can use ? as well; if main returns an
 * Err, Rust prints it to stderr and exits with a non-zero status.
 */
use std::fs;
use std::io;

fn read_username() -> Result<String, io::Error> {
    // The ? operator propagates the io::Error if the file cannot be read.
    let contents = fs::read_to_string("username.txt")?;
    Ok(contents.trim().to_string())
}

fn main() -> Result<(), io::Error> {
    let username = read_username()?;
    println!("username is {username}");
    Ok(())
}
