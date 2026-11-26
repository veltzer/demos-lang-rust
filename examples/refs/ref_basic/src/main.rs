fn main() {
    let x = 5;
    let y = &x;

    println!("The value of x is: {x}");
    println!("The value of y (reference to x) is: {y}");

    print_number(&x);
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "teaching: the example is about passing a reference")]
fn print_number(num: &i32) {
    println!("The number is: {num}");
}
