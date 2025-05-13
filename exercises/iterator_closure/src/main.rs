fn create_iterator(start: i32, stop: i32, step: i32) -> impl Iterator<Item = i32> {
    let mut current = start;
    std::iter::from_fn(move || {
        if current < stop {
            let value = current;
            current += step;
            Some(value)
        } else {
            None
        }
    })
}

fn main() {
    let iterator = create_iterator(1, 10, 2);
    
    for value in iterator {
        println!("{}", value);
    }
}
