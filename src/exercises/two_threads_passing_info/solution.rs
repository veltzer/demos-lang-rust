use std::sync::mpsc::*;
use std::thread;
// use std::time::Duration;
use std::io::*;

//
// This is a solution to the exercise.
//

fn thread_one(tx: Sender<i32>) {
    print!("thread_one: Give me a number: ");
    stdout().flush().unwrap();
    for line in stdin().lines() {
        let x: i32 = line.unwrap().trim().parse().unwrap();
        tx.send(x).unwrap();
        //thread::sleep(Duration::from_secs(1));
        print!("thread_one: Give me a number: ");
        stdout().flush().unwrap();
    }
}

fn thread_two(rx: Receiver<i32>) {
    for msg in rx {
        println!("thread_two: Got: {msg}");
    }
}

fn main() {
    let (tx, rx) = channel::<i32>();
    let thread_join_handle1 = thread::spawn(|| {thread_one(tx); });
    let thread_join_handle2 = thread::spawn(|| {thread_two(rx); });
    thread_join_handle1.join().unwrap();
    thread_join_handle2.join().unwrap();
}
