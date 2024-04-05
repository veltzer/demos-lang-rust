// This is an example of how to use closures to pass parameters
// to the thread worker function in Rust.

use std::thread;
use std::time::Duration;

fn do_thread(name: &str) {
    for i in 1..10 {
        println!("{name}: number {i} from the spawned thread!");
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let thread_join_handle1 = thread::spawn(|| {do_thread("one"); });
    let thread_join_handle2 = thread::spawn(|| {do_thread("two"); });
    thread_join_handle1.join().unwrap();
    thread_join_handle2.join().unwrap();
}
