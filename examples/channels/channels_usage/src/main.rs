use std::thread;
use std::sync::mpsc;

fn main() {
    // Create a new channel
    let (tx, rx) = mpsc::channel();

    // Clone the transmitter for the second thread
    let tx1 = tx.clone();

    // Spawn a new thread
    thread::spawn(move || {
        // Send a message from this thread
        tx.send("Hello from thread 1").unwrap();
    });

    // Spawn another thread
    thread::spawn(move || {
        // Send a message from this thread
        tx1.send("Hello from thread 2").unwrap();
    });

    // Receive messages in the main thread
    for _ in 0..2 {
        // The recv() method blocks until a message is received
        let message = rx.recv().unwrap();
        println!("Got: {}", message);
    }
}
