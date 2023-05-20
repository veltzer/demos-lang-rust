// This is a simple exapmle of multh-threading in rust
// you usually want to join your threads and not leave them
// in a detached state.

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
