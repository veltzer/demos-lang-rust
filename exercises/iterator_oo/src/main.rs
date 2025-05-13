// Custom range iterator that yields numbers from start to end-1
struct RangeIterator {
    current: i32,
    end: i32,
}

// Constructor for our iterator
impl RangeIterator {
    fn new(start: i32, end: i32) -> Self {
        RangeIterator {
            current: start,
            end: end,
        }
    }
}

// Implementation of the Iterator trait
impl Iterator for RangeIterator {
    // Define the type of item this iterator will produce
    type Item = i32;
    
    // The required next() method that returns the next value
    // or None when iteration is finished
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            // Store the current value
            let result = self.current;
            // Advance the iterator state
            self.current += 1;
            // Return the value
            Some(result)
        } else {
            // No more values to yield
            None
        }
    }
}

// A container type that will provide our iterator
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Range { start, end }
    }
    
    // Method that returns our iterator
    fn iter(&self) -> RangeIterator {
        RangeIterator::new(self.start, self.end)
    }
}

fn main() {
    // Create a range from 1 to 5
    let range = Range::new(1, 6);
    
    // Use a for loop with our iterator
    println!("Using for loop:");
    for num in range.iter() {
        println!("{}", num);
    }
    
    // Create another range and use iterator methods directly
    let range2 = Range::new(10, 15);
    println!("\nUsing iterator methods:");
    let sum: i32 = range2.iter().sum();
    println!("Sum: {}", sum);
    
    // Manual iteration with while let
    let range3 = Range::new(5, 8);
    println!("\nManual iteration:");
    let mut iter = range3.iter();
    while let Some(value) = iter.next() {
        println!("{}", value);
    }
}
