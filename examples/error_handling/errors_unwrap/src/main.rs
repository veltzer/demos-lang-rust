/*
 * This example shows the .unwrap() method of the Result and Option types.
 *
 * .unwrap() extracts the value inside:
 * - for Result: Ok(v) yields v, and Err(e) panics with a message containing e
 * - for Option: Some(v) yields v, and None panics
 *
 * .unwrap() is convenient but crashes the program on failure, so it is best
 * used in examples, prototypes, and tests, or when you can prove the value is
 * present. In production code prefer matching, .expect() with a message, or
 * the ? operator to propagate the error.
 */
fn main() {
    // Result::unwrap on the Ok path: parsing a valid number succeeds.
    let good: i32 = "42".parse().unwrap();
    println!("parsed the number {good}");

    // Option::unwrap on the Some path: the vector has a first element.
    let numbers = vec![1, 2, 3];
    let first = numbers.first().unwrap();
    println!("first element is {first}");

    // Result::unwrap on the Err path: this string is not a number, so
    // .unwrap() panics and prints the parse error.
    let bad: i32 = "not a number".parse().unwrap();
    println!("this line is never reached: {bad}");
}
