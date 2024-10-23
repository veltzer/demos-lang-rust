#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    isbn: String,
}

#[derive(Debug)]
struct Member {
    id: u32,
    name: String,
    borrowed_books: Vec<Book>,
}

#[derive(Debug)]
struct Library {
    available_books: Vec<Book>,
    members: Vec<Member>,
}

impl Library {
    fn new() -> Self {
        Library {
            available_books: Vec::new(),
            members: Vec::new(),
        }
    }

    // Takes ownership of the book
    fn add_book(&mut self, book: Book) {
        self.available_books.push(book);
    }

    // Creates and owns a new member
    fn register_member(&mut self, name: String) -> u32 {
        let id = self.members.len() as u32 + 1;
        self.members.push(Member {
            id,
            name,
            borrowed_books: Vec::new(),
        });
        id
    }

    // Demonstrates ownership transfer of a book from library to member
    fn check_out_book(&mut self, member_id: u32, isbn: &str) -> Result<(), String> {
        // Find the member - mutable borrow
        let member = self.members
            .iter_mut()
            .find(|m| m.id == member_id)
            .ok_or("Member not found")?;

        // Find the book index - immutable borrow
        let book_idx = self.available_books
            .iter()
            .position(|book| book.isbn == isbn)
            .ok_or("Book not available")?;

        // Remove the book from available_books (ownership transfer)
        let book = self.available_books.remove(book_idx);
        
        // Give the book to the member (ownership transfer)
        member.borrowed_books.push(book);
        
        Ok(())
    }

    // Demonstrates ownership transfer of a book from member back to library
    fn return_book(&mut self, member_id: u32, isbn: &str) -> Result<(), String> {
        // Find the member - mutable borrow
        let member = self.members
            .iter_mut()
            .find(|m| m.id == member_id)
            .ok_or("Member not found")?;

        // Find the book index in member's borrowed books
        let book_idx = member.borrowed_books
            .iter()
            .position(|book| book.isbn == isbn)
            .ok_or("Book not found in member's borrowed books")?;

        // Remove the book from member's borrowed_books (ownership transfer)
        let book = member.borrowed_books.remove(book_idx);
        
        // Add the book back to available_books (ownership transfer)
        self.available_books.push(book);
        
        Ok(())
    }

    // Demonstrates borrowing for reading data
    fn get_member_books(&self, member_id: u32) -> Option<&Vec<Book>> {
        self.members
            .iter()
            .find(|m| m.id == member_id)
            .map(|member| &member.borrowed_books)
    }

    // Demonstrates working with references to create a report
    fn generate_borrowing_report(&self) -> Vec<(u32, String, usize)> {
        self.members
            .iter()
            .map(|member| (member.id, member.name.clone(), member.borrowed_books.len()))
            .collect()
    }
}

fn main() {
    // Create a new library
    let mut library = Library::new();

    // Add books (ownership transfer to library)
    library.add_book(Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik"),
        isbn: String::from("123456789"),
    });
    
    library.add_book(Book {
        title: String::from("Programming Rust"),
        author: String::from("Jim Blandy"),
        isbn: String::from("987654321"),
    });

    // Register a member (library owns the member data)
    let member_id = library.register_member(String::from("Alice Johnson"));

    println!("Initial library state:");
    println!("Available books: {}", library.available_books.len());

    // Check out a book (ownership transfers from library to member)
    match library.check_out_book(member_id, "123456789") {
        Ok(_) => println!("Book 'The Rust Programming Language' checked out successfully"),
        Err(e) => println!("Error checking out book: {}", e),
    }

    // View borrowed books (borrowing member's data)
    if let Some(books) = library.get_member_books(member_id) {
        println!("Member has {} books after checkout", books.len());
    }

    // Generate report (borrowing data for reading)
    println!("\nCurrent borrowing report:");
    let report = library.generate_borrowing_report();
    for (id, name, count) in report {
        println!("Member {} ({}) has {} books", name, id, count);
    }

    // Return the book (ownership transfers back to library)
    match library.return_book(member_id, "123456789") {
        Ok(_) => println!("\nBook 'The Rust Programming Language' returned successfully"),
        Err(e) => println!("\nError returning book: {}", e),
    }

    // View updated borrowed books
    if let Some(books) = library.get_member_books(member_id) {
        println!("Member has {} books after return", books.len());
    }

    // Show final library state
    println!("Final available books: {}", library.available_books.len());

    // Try to return a book that wasn't borrowed (should show error handling)
    match library.return_book(member_id, "nonexistent") {
        Ok(_) => println!("Book returned successfully"),
        Err(e) => println!("Expected error occurred: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_book() -> Book {
        Book {
            title: String::from("Test Book"),
            author: String::from("Test Author"),
            isbn: String::from("TEST123"),
        }
    }

    #[test]
    fn test_book_checkout_and_return() {
        let mut library = Library::new();
        let book = create_test_book();
        library.add_book(book.clone());
        let member_id = library.register_member(String::from("Test Member"));

        // Test checkout
        assert!(library.check_out_book(member_id, "TEST123").is_ok());
        assert_eq!(library.available_books.len(), 0);
        
        // Verify member has the book
        let member_books = library.get_member_books(member_id).unwrap();
        assert_eq!(member_books.len(), 1);
        assert_eq!(member_books[0], book);

        // Test return
        assert!(library.return_book(member_id, "TEST123").is_ok());
        assert_eq!(library.available_books.len(), 1);
        
        // Verify member no longer has the book
        let member_books = library.get_member_books(member_id).unwrap();
        assert_eq!(member_books.len(), 0);
    }

    #[test]
    fn test_invalid_checkout() {
        let mut library = Library::new();
        let member_id = library.register_member(String::from("Test Member"));

        // Try to check out non-existent book
        assert!(library.check_out_book(member_id, "NONEXISTENT").is_err());
    }

    #[test]
    fn test_invalid_return() {
        let mut library = Library::new();
        let member_id = library.register_member(String::from("Test Member"));

        // Try to return non-borrowed book
        assert!(library.return_book(member_id, "NONEXISTENT").is_err());
    }

    #[test]
    fn test_borrowing_report() {
        let mut library = Library::new();
        let member_id = library.register_member(String::from("Test Member"));
        let book = create_test_book();
        library.add_book(book);
        
        // Check out a book
        library.check_out_book(member_id, "TEST123").unwrap();
        
        // Verify report
        let report = library.generate_borrowing_report();
        assert_eq!(report.len(), 1);
        assert_eq!(report[0].0, member_id);
        assert_eq!(report[0].2, 1); // Number of borrowed books
    }
}
