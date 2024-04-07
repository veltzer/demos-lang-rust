#![allow(unused_must_use)]

use std::io;

// #[allow(unused_must_use)]
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess); // no error handling here

    println!("You guessed: {guess}");
}
