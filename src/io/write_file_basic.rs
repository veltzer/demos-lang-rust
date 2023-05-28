// This is a basic example of how to write fomatted data to a file in Rust
// Rust will take care of closing the file

use std::fs::File;
use std::io::Write;

fn main() {
    let mut w = File::create("/tmp/test.txt").unwrap();
    writeln!(&mut w, "formatted {}", "arguments").unwrap();
}
