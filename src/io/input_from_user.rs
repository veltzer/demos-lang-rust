// This is a simple exapmle of how to read input from the user

use std::io::{stdout,stdin,Write};

fn main() {
    print!("What is your name: ");
    stdout().flush().expect("canont flush stdout");
    let mut name: String = String::new();
    stdin().read_line(&mut name).expect("Did not enter a correct string");
    name = name.strip_suffix("\n").unwrap().to_string();
    println!("Your name is [{name}]...");
}
