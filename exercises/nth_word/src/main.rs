use std::io::{self, Write};

fn main() {
    // Get the input string from user
    print!("Enter a string: ");
    io::stdout().flush().unwrap(); // Flush to ensure prompt appears
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    // Get the position (n) from user
    print!("Enter the word position (n): ");
    io::stdout().flush().unwrap();
    
    let mut n_input = String::new();
    io::stdin()
        .read_line(&mut n_input)
        .expect("Failed to read line");
    
    // Parse n, with error handling
    let n: usize = match n_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };
    
    // Call the function to find and print the nth word
    find_nth_word(&input, n);
}

/// Finds and prints the nth word in a string slice
fn find_nth_word(text: &str, n: usize) {
    // Split the text into words
    let words: Vec<&str> = text.split_whitespace().collect();
    
    // Check if n is within range
    if n == 0 {
        println!("Word positions start at 1!");
    } else if n <= words.len() {
        // Print the nth word (adjusting for 0-based indexing)
        println!("The word at position {} is: \"{}\"", n, words[n-1]);
    } else {
        println!("Error: The text contains only {} words!", words.len());
    }
}
