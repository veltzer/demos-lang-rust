use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    // Create an atomic counter
    let counter = Arc::new(AtomicUsize::new(0));
    let num_threads = 10;
    let iterations_per_thread = 100_000;

    let mut handles = vec![];

    // Spawn multiple threads
    for _ in 0..num_threads {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..iterations_per_thread {
                // Increment the counter atomically
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    println!("Final counter value: {}", counter.load(Ordering::SeqCst));

    // Check if the final value matches the expected value
    let expected_value = num_threads * iterations_per_thread;
    assert_eq!(counter.load(Ordering::SeqCst), expected_value, 
               "Counter value doesn't match expected value!");
    
    println!("Test passed! The atomic operation worked correctly.");
}
