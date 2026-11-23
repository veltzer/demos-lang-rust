use std::io;

#[expect(unused_must_use, reason = "teaching: shows how to disregard a Result that must be used")]
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess); // no error handling here

    println!("You guessed: {guess}");
}
