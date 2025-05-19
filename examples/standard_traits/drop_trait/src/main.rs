// Define a struct that simulates holding a file resource.
struct FileHandle {
    name: String,
    is_open: bool,
}

impl FileHandle {
    // Constructor for our FileHandle
    fn new(name: &str) -> FileHandle {
        println!("Opening file: {}", name);
        FileHandle {
            name: String::from(name),
            is_open: true, // Simulate the file being opened
        }
    }

    // A method to check if the file is considered open
    #[allow(dead_code)] // Allow this method to be unused for this example
    fn is_open(&self) -> bool {
        self.is_open
    }
}

// Implement the Drop trait for FileHandle
impl Drop for FileHandle {
    // The drop method is called automatically when an instance of FileHandle goes out of scope.
    fn drop(&mut self) {
        if self.is_open {
            // This is where you would put the actual resource cleanup logic,
            // e.g., closing a file, releasing a lock, closing a network connection.
            println!("Closing file: {} (Dropping FileHandle)", self.name);
            self.is_open = false; // Mark the file as closed
        } else {
            // This case might occur if the resource was already manually released
            // or if drop is somehow called more than once (which Rust prevents for safe code).
            println!("File {} was already closed. (Dropping FileHandle)", self.name);
        }
    }
}

// Another struct to demonstrate drop order and explicit drops
struct NoisyDropper {
    id: i32,
    message: String,
}

impl NoisyDropper {
    fn new(id: i32, message: &str) -> NoisyDropper {
        println!("NoisyDropper {} created with message: '{}'", id, message);
        NoisyDropper {
            id,
            message: String::from(message),
        }
    }
}

impl Drop for NoisyDropper {
    fn drop(&mut self) {
        println!(
            "NoisyDropper {} is being dropped! Message: '{}'",
            self.id, self.message
        );
    }
}

fn main() {
    println!("--- Program Start ---");

    // Create a FileHandle instance.
    // It will be dropped when it goes out of scope at the end of `main`.
    let file1 = FileHandle::new("important_data.txt");
    println!("FileHandle for '{}' created. Is open: {}", file1.name, file1.is_open());

    // Demonstrate nested scopes and drop order
    {
        println!("\n--- Entering inner scope ---");
        let file2 = FileHandle::new("temporary_log.txt");
        let _noisy1 = NoisyDropper::new(1, "I live in the inner scope");
        println!("Created file2 ('{}') and noisy1 in inner scope.", file2.name);
        // At the end of this inner scope, noisy1 will be dropped first, then file2,
        // because variables are dropped in reverse order of their declaration.
        println!("--- Leaving inner scope ---");
    } // file2 and noisy1 go out of scope here.

    println!("\n--- Back in main scope ---");

    let noisy2 = NoisyDropper::new(2, "I might be dropped early!");
    println!("Created noisy2.");

    // Explicitly drop noisy2 using std::mem::drop
    // This is useful if you need to release a resource before the variable
    // would naturally go out of scope.
    println!("Attempting to explicitly drop noisy2...");
    std::mem::drop(noisy2); // noisy2's drop method is called here.
                           // `noisy2` cannot be used after this line as it has been moved and dropped.
    println!("noisy2 should have been dropped.");

    // This would cause a compile error because noisy2 has been moved and dropped:
    // println!("Trying to use noisy2 after explicit drop: {}", noisy2.id);

    println!("\n--- Program End ---");
    // file1 goes out of scope here at the end of main.
}
