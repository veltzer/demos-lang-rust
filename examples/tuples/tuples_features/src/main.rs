fn main() {
    // 1. Basic tuple creation and destructuring
    let person = ("Alice", 30, "Software Engineer");
    let (name, age, job) = person;
    println!("1. Basic destructuring: {} is {} years old and works as a {}.", name, age, job);

    // 2. Ignoring tuple values
    let (name, _, job) = person;
    println!("2. Ignoring values: {} works as a {}.", name, job);

    // 3. Partial destructuring
    let (name, ..) = person;
    println!("3. Partial destructuring: Name is {}.", name);

    // 4. Tuple indexing
    println!("4. Tuple indexing: Age is {}.", person.1);

    // 5. Tuples as return values
    fn calculate_stats(numbers: &[i32]) -> (i32, i32, f64) {
        let sum: i32 = numbers.iter().sum();
        let count = numbers.len() as i32;
        let average = sum as f64 / count as f64;
        (sum, count, average)
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let (sum, count, average) = calculate_stats(&numbers);
    println!("5. Tuples as return values: Sum: {}, Count: {}, Average: {:.2}", sum, count, average);

    // 6. Nested tuple destructuring
    let nested = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;
    println!("6. Nested destructuring: a={}, b={}, c={}, d={}", a, b, c, d);

    // 7. Using tuples with generic types
    fn swap<T, U>(tuple: (T, U)) -> (U, T) {
        let (first, second) = tuple;
        (second, first)
    }

    let swapped = swap((1, "hello"));
    println!("7. Generic tuple swap: {:?}", swapped);

    // 8. Tuples in pattern matching
    fn describe_point(point: (i32, i32)) -> &'static str {
        match point {
            (0, 0) => "Origin",
            (0, _) => "On the y-axis",
            (_, 0) => "On the x-axis",
            (x, y) if x == y => "On the diagonal",
            _ => "Just a regular point",
        }
    }

    println!("8. Pattern matching with tuples:");
    println!("  (0, 0) is {}", describe_point((0, 0)));
    println!("  (0, 10) is {}", describe_point((0, 10)));
    println!("  (10, 0) is {}", describe_point((10, 0)));
    println!("  (5, 5) is {}", describe_point((5, 5)));
    println!("  (3, 7) is {}", describe_point((3, 7)));

    // 9. Tuple structs
    struct Color(i32, i32, i32);
    let red = Color(255, 0, 0);
    println!("9. Tuple struct: Red value is {}", red.0);
}
