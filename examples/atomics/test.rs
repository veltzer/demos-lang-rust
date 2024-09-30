use std::thread::{JoinHandle,spawn};

fn worker() {
    println!("in worker");
}

const NUM_THREADS: i32=8;

fn main() {
    let v: Vec<JoinHandle<()>> = (0..NUM_THREADS).map(|_| spawn(worker) ).collect();
    for t in v {
         t.join().unwrap();
    }
    // _ = v.into_iter().map(|t| t.join().unwrap());
    //v.for_each(|t| t.join().unwrap());
    //v.iter_mut().map(|t| t.join().unwrap());
}
