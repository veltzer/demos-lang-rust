// This example shows that overflow is not detected

fn main() {
    let mut x: i8=127;
    println!("x is {x}");
    x=x+1;
    println!("x is {x}");
}
