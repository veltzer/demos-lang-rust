use std::io;
use std::io::{Write, Error};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!");
    
    // Generate random number between 0 and 100
    let secret_number = rand::thread_rng().gen_range(0..101);
    
    loop {
        print!("Please input your guess (0-100): ");
        io::stdout().flush().unwrap();
        
        // Create a new String to store the guess
        let mut guess: String = String::new();
        
        // Read user input
        let _r :Result<usize, Error > = io::stdin().read_line(&mut guess);
            //.expect("Failed to read line");
        
        // Convert guess to number, handle invalid input
        let guess: i32 = match guess.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        // Check if guess is in valid range
        if guess < 0 || guess > 100 {
            println!("Please enter a number between 0 and 100!");
            continue;
        }
        
        // Compare guess with secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! Try again."),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number!");
                break;
            }
        }
    }
}
