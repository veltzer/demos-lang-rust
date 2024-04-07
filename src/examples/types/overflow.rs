// This example shows that overflow is not detected

fn main() {
    let mut x: u8=255;
    println!("x is {x}");
    x=x+1;
    println!("x is {x}");
}
