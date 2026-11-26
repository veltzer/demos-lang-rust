use std::time::Instant;

fn xor_1000_ints() -> u32 {
    (0..1000).fold(0, |old, new| old ^ new)
}

#[expect(clippy::needless_pass_by_value, reason = "teaching: a hand-written black_box takes its value to hide it from the optimizer")]
fn black_box<T>(dummy: T) -> T {
    unsafe { std::ptr::read_volatile(&raw const dummy) }
}

fn main() {
    // Run the function and measure its execution time
    let start = Instant::now();
    let result = xor_1000_ints();
    let duration = start.elapsed();

    println!("Result: {result}");
    println!("Time taken: {duration:?}");

    // Run it multiple times to get a sense of the average performance
    let iterations = 1000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(xor_1000_ints());
    }
    let duration = start.elapsed();

    println!("Average time over {} iterations: {:?}", iterations, duration / iterations);
}
