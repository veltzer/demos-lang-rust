fn main() {
    // Input data: numbers from 1 to 5
    let data: Vec<i32> = (1..=5).collect();
    
    // Map phase: Square each number
    let mapped: Vec<i32> = data.into_iter()
        .map(|x| x * x)
        .collect();
    
    // Reduce phase: Sum all squared numbers
    let result: i32 = mapped.into_iter()
        .reduce(|acc, x| acc + x)
        .unwrap();
    
    println!("Sum of squares from 1 to 5: {}", result);
    // Expected output: 55 (1² + 2² + 3² + 4² + 5² = 1 + 4 + 9 + 16 + 25 = 55)
}
