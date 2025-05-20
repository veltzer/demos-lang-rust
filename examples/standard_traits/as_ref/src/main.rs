fn main() {
    // Function works with different string types via AsRef<str>
    print_length(String::from("Hello"));  // String
    print_length("World");               // &str
    print_length(&String::from("Rust")); // &String
    
    // Custom type with AsRef implementation
    let person = Person { name: String::from("Alice") };
    print_length(&person);  // Works because Person implements AsRef<str>
}

// Generic function accepting anything that can be viewed as &str
fn print_length<T: AsRef<str>>(s: T) {
    let s_ref = s.as_ref(); // Convert to &str
    println!("\"{}\" has length: {}", s_ref, s_ref.len());
}

// Custom type
struct Person {
    name: String,
}

// Implement AsRef<str> for our type
impl AsRef<str> for Person {
    fn as_ref(&self) -> &str {
        &self.name
    }
}
