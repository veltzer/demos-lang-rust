fn main() {
    // Example 1: Sum of all elements in a vector
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum of {:?} is {}", numbers, sum);

    // Example 2: Finding the maximum value in a vector
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let max = numbers.iter().fold(std::i32::MIN, |acc, &x| acc.max(x));
    println!("Maximum value in {:?} is {}", numbers, max);

    // Example 3: Concatenating strings
    let words = vec!["Hello", "world", "from", "Rust"];
    let sentence = words.iter().fold(String::new(), |acc, &x| acc + x + " ");
    println!("Concatenated string: {}", sentence.trim());

    // Example 4: Custom struct reduction
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let points = vec![
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
        Point { x: 5, y: 6 },
    ];

    let sum_point = points.iter().fold(Point { x: 0, y: 0 }, |acc, p| Point {
        x: acc.x + p.x,
        y: acc.y + p.y,
    });

    println!("Sum of all points: {:?}", sum_point);

    // Example 5: Counting occurrences of elements
    let chars = vec!['a', 'b', 'a', 'c', 'b', 'a'];
    let char_counts = chars.iter().fold(std::collections::HashMap::new(), |mut acc, &c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    println!("Character counts: {:?}", char_counts);
}
