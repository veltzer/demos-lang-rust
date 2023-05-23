// This is an example of how to do fork in rust

// References:
// - https://docs.rs/fork/latest/fork/

extern crate fork;

use fork::{daemon, Fork};
use std::process::Command;


fn main() {
    println!("Hello from the parent");
	if let Ok(Fork::Child) = daemon(false, false) {
	   Command::new("sleep")
		   .arg("3")
		   .output()
		   .expect("failed to execute process");
	}
}
