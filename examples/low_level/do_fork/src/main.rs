// This is an example of how to do fork in rust

// References:
// - https://docs.rs/fork/latest/fork/

extern crate fork;

use fork::{daemon, Fork};
use std::process::Command;


fn main() {
    println!("Hello from the parent");
	if matches!(daemon(false, false), Ok(Fork::Child)) {
	   Command::new("sleep")
		   .arg("3")
		   .output()
		   .unwrap();
	}
}
