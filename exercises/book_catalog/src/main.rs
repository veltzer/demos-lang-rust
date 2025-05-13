use std::fmt;

// Define the Book struct
struct Book {
    title: String,
    author: String,
}

// Implement methods for Book
impl Book {
    // Constructor method
    fn new(title: &str, author: &str) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
        }
    }
}

// Implement the Display trait for pretty printing
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\" by {}", self.title, self.author)
    }
}

fn main() {
    // Create some book instances
    let book1 = Book::new("The Rust Programming Language", "Steve Klabnik and Carol Nichols");
    let book2 = Book::new("Rust in Action", "Tim McNamara");
    
    // Print the books using the Display trait implementation
    println!("My book catalog:");
    println!("1. {}", book1);
    println!("2. {}", book2);
}
