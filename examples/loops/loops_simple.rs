// This is an example of how to flush stdout in rust

use std::thread::sleep;
use std::time::Duration;

fn main() {
    for n in 0..10 {
        println!("n is {n}");
        sleep(Duration::from_secs(1));
    }
}
