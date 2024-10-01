fn main() {
    // 1. Iterating over a range
    println!("1. Iterating over a range:");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!("\n");

    // 2. Iterating over an inclusive range
    println!("2. Iterating over an inclusive range:");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!("\n");

    // 3. Iterating over a collection (vector)
    println!("3. Iterating over a vector:");
    let numbers = vec![1, 2, 3, 4, 5];
    for num in &numbers {
        print!("{} ", num);
    }
    println!("\n");

    // 4. Iterating with an index
    println!("4. Iterating with an index:");
    for (index, value) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
    println!();

    // 5. Iterating over a string's characters
    println!("5. Iterating over a string's characters:");
    for c in "Hello, 世界!".chars() {
        print!("{} ", c);
    }
    println!("\n");

    // 6. Iterating over array
    println!("6. Iterating over an array:");
    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        print!("{} ", element);
    }
    println!("\n");

    // 7. Iterating over a mutable collection
    println!("7. Iterating over a mutable collection:");
    let mut mutable_numbers = vec![1, 2, 3, 4, 5];
    for num in mutable_numbers.iter_mut() {
        *num *= 2;
        print!("{} ", num);
    }
    println!("\n");

    // 8. Using loop labels
    println!("8. Using loop labels:");
    'outer: for x in 1..=3 {
        for y in 1..=3 {
            if x == 2 && y == 2 {
                break 'outer;
            }
            println!("({}, {})", x, y);
        }
    }
    println!();

    // 9. Iterating over a HashMap
    println!("9. Iterating over a HashMap:");
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("apple", 1);
    map.insert("banana", 2);
    map.insert("cherry", 3);
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
