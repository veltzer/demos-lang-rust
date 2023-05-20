// This is an example of how to flush stdout in rust

use std::thread::sleep;
use std::time::Duration;
use std::io::stdout;
use std::io::Write;

fn main() {
    for n in 0..10 {
        print!("{n:>3}\r");
        stdout().flush().expect("cannot flush stdout");
        sleep(Duration::from_secs(1));
    }
}
