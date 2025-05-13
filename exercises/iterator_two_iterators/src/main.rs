struct ChainedIterator<'a> {
    // Store iterators for both vectors
    first_iter: std::slice::Iter<'a, i32>,
    second_iter: std::slice::Iter<'a, i32>,
    // Track which iterator we're currently using
    using_first: bool,
}

impl<'a> ChainedIterator<'a> {
    // Constructor that takes references to two vectors
    fn new(first: &'a Vec<i32>, second: &'a Vec<i32>) -> Self {
        ChainedIterator {
            first_iter: first.iter(),
            second_iter: second.iter(),
            using_first: true,
        }
    }
}

// Implement the Iterator trait
impl<'a> Iterator for ChainedIterator<'a> {
    // The type of items our iterator yields
    type Item = &'a i32;
    
    // The required next method that returns the next value or None when done
    fn next(&mut self) -> Option<Self::Item> {
        if self.using_first {
            // Try to get the next item from the first iterator
            match self.first_iter.next() {
                Some(value) => Some(value),
                None => {
                    // First iterator is exhausted, switch to the second
                    self.using_first = false;
                    self.second_iter.next()
                }
            }
        } else {
            // Already using the second iterator
            self.second_iter.next()
        }
    }
}

fn main() {
    // Create two sample vectors
    let first_vec = vec![1, 2, 3];
    let second_vec = vec![4, 5, 6];
    
    // Create our chained iterator
    let chained = ChainedIterator::new(&first_vec, &second_vec);
    
    // Use the iterator in a for loop
    println!("Iterating through chained vectors:");
    for value in chained {
        println!("{}", value);
    }
}
