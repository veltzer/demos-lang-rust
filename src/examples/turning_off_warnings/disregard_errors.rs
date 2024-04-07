use std::io;

// This example shows that the _ special variable name is the name to assign
// to if you don't care about the result.
// Giving the variable a name will result in an unused variable warning.

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    let _ = io::stdin()
        .read_line(&mut guess); // no error handling here

    println!("You guessed: {guess}");
}
