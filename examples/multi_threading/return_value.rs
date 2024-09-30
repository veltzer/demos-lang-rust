// This is an example of how a thread can return a value
// This is something very unique to rust as in most other languages
// threads cannot return a value.

use std::thread;

fn do_thread() -> i32 {
    7
}

fn main() {
    let thread_join_handle = thread::spawn(do_thread);
    let r = thread_join_handle.join().unwrap();
    println!("value is {r}");
}
