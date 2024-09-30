fn main() {
    let x = 5;
    let y = &x;

    println!("The value of x is: {}", x);
    println!("The value of y (reference to x) is: {}", y);

    print_number(&x);
}

fn print_number(num: &i32) {
    println!("The number is: {}", num);
}
