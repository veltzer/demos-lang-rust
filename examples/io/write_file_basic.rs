// This is a basic example of how to write fomatted data to a file in Rust
// Rust will take care of closing the file
//
// References:
// - https://stackoverflow.com/questions/32472495/how-do-i-write-a-formatted-string-to-a-file

use std::fs::File;
use std::io::Write;

fn main() {
    let mut w = File::create("/tmp/test.txt").unwrap();
    writeln!(&mut w, "formatted {}", "arguments").unwrap();
}
