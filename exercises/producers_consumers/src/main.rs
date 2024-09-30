use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use rand::Rng;
use crossbeam_channel::{bounded, Sender, Receiver};

fn producer(id: usize, tx: Sender<i32>) {
    let mut rng = rand::thread_rng();
    loop {
        let number: i32 = rng.gen();
        if tx.send(number).is_err() {
            break;
        }
        println!("Producer {} produced: {}", id, number);
        thread::sleep(Duration::from_millis(1));
    }
}

fn consumer(id: usize, rx: Receiver<i32>) {
    let mut file = File::create("/dev/null").expect("Failed to open /dev/null");
    while let Ok(number) = rx.recv() {
        writeln!(file, "{}", number).expect("Failed to write to /dev/null");
        println!("Consumer {} consumed: {}", id, number);
    }
}

fn main() {
    let n_producers = 3;
    let m_consumers = 2;
    let (tx, rx) = bounded(100);  // Channel with a buffer of 100 messages

    let mut handles = vec![];

    // Spawn producer threads
    for i in 0..n_producers {
        let tx = tx.clone();
        handles.push(thread::spawn(move || producer(i, tx)));
    }

    // Spawn consumer threads
    for i in 0..m_consumers {
        let rx = rx.clone();
        handles.push(thread::spawn(move || consumer(i, rx)));
    }

    // Wait for all threads to complete (this will never happen in this case)
    for handle in handles {
        handle.join().unwrap();
    }
}
