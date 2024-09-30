use std::fs::File;
use std::io::*;

/*
 * This is an example of how to read a file line by line in rust
 *
 * Notes:
 * - you must "unwrap" both the BufferedReader and the line to use them.
 *
 * References:
 * - https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
 */

fn main() {
    let file = File::open("/etc/passwd").unwrap();
    let lines = BufReader::new(file).lines(); 
    for line in lines {
        println!("{}", line.unwrap());
    }
}
