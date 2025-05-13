Explanation
This solution demonstrates several key concepts of Rust structs:

Struct Definition: We defined a Book struct with two String fields for title and author.
Implementation Block: The impl Book block adds methods to the struct:

The new function creates a new Book instance with the provided title and author
It serves as a constructor and converts the string slices to owned Strings


Display Trait: By implementing the fmt::Display trait, we enable formatted printing of our struct using the {} placeholder in println!.
Instance Creation: In the main function, we create two book instances using our constructor method.
Using the struct: We print our books using the Display implementation, which formats them nicely.

This pattern is very common in Rust - defining a data structure and then implementing behavior for it separately. You could extend this example by adding more fields like publication year, genre, or by creating a Library struct that contains multiple books.
