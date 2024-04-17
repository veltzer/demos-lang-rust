/*
 * This example shows that you cannot use a slice after the array it
 * is connected too has been mutated or dropped.
 */

fn get_slice(a: &[i32]) -> &[i32] {
    for (i, num) in a.iter().enumerate() {
        if num % 5 == 0 {
            return &a[0..i];
        }
    }
    &a[..]
}

fn main() {
    let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    a[6] += 1;
    let slice = get_slice(&a);
    println!("slice is: {:?}", slice);
    // this mutation causes the slice to be un-usable
    a[6] += 1;
    // println!("slice is: {:?}", slice); // <--- this causes a compilation error
    println!("a is: {:?}", a);
}
