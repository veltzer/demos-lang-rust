use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct TaskQueue {
    tasks: Vec<AtomicBool>,
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl TaskQueue {
    fn new(size: usize) -> Self {
        TaskQueue {
            tasks: (0..size).map(|_| AtomicBool::new(false)).collect(),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    fn enqueue(&self) -> bool {
        let mut tail = self.tail.load(Ordering::Relaxed);
        loop {
            let next_tail = (tail + 1) % self.tasks.len();
            if next_tail == self.head.load(Ordering::Relaxed) {
                return false; // Queue is full
            }
            match self.tail.compare_exchange_weak(tail, next_tail, Ordering::SeqCst, Ordering::Relaxed) {
                Ok(_) => {
                    self.tasks[tail].store(true, Ordering::Release);
                    return true;
                }
                Err(new_tail) => tail = new_tail,
            }
        }
    }

    fn dequeue(&self) -> bool {
        let mut head = self.head.load(Ordering::Relaxed);
        loop {
            if head == self.tail.load(Ordering::Relaxed) {
                return false; // Queue is empty
            }
            if self.tasks[head].load(Ordering::Acquire) {
                let next_head = (head + 1) % self.tasks.len();
                match self.head.compare_exchange_weak(head, next_head, Ordering::SeqCst, Ordering::Relaxed) {
                    Ok(_) => {
                        self.tasks[head].store(false, Ordering::Release);
                        return true;
                    }
                    Err(new_head) => head = new_head,
                }
            } else {
                return false; // No task available
            }
        }
    }
}

fn main() {
    let queue = Arc::new(TaskQueue::new(10));
    let num_producers = 3;
    let num_consumers = 2;
    let mut handles = vec![];

    // Spawn producer threads
    for i in 0..num_producers {
        let queue = Arc::clone(&queue);
        handles.push(thread::spawn(move || {
            for j in 0..5 {
                while !queue.enqueue() {
                    thread::yield_now();
                }
                println!("Producer {} enqueued task {}", i, j);
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }

    // Spawn consumer threads
    for i in 0..num_consumers {
        let queue = Arc::clone(&queue);
        handles.push(thread::spawn(move || {
            let mut tasks_processed = 0;
            while tasks_processed < 15 / num_consumers {
                if queue.dequeue() {
                    tasks_processed += 1;
                    println!("Consumer {} processed a task", i);
                    thread::sleep(Duration::from_millis(2));
                } else {
                    thread::yield_now();
                }
            }
        }));
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All tasks have been processed!");
}
