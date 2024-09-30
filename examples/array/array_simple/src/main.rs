fn main() {
    // Declare and initialize an array of integers
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Print the entire array
    println!("The array is: {:?}", numbers);

    // Access individual elements
    println!("The first element is: {}", numbers[0]);
    println!("The last element is: {}", numbers[4]);

    // Get the length of the array
    println!("The length of the array is: {}", numbers.len());

    // Iterate over the array
    println!("Iterating over the array:");
    for num in numbers.iter() {
        println!("  {}", num);
    }

    // Declare an array with default values
    let zeros = [0; 5];
    println!("Array of zeros: {:?}", zeros);

    // Modify elements in a mutable array
    let mut mutable_array = [1, 2, 3, 4, 5];
    mutable_array[2] = 10;
    println!("Modified array: {:?}", mutable_array);

    // Using array slices
    let slice = &numbers[1..4];
    println!("Slice of the array: {:?}", slice);
}
