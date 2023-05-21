// This is an example of how to use a closure to comply with some "contract"
// you have an add() function but you need a function that keeps one of it's
// variables constant and only sends the second one.

fn add(a: i32, b: i32) -> i32 {
    a+b
}

fn make_adder(outer: i32) -> impl Fn(i32) -> i32 {
    move |inner: i32| add(outer,inner)
}

// simple example of function calling a function
fn main() {
    let a1 = make_adder(3);
    let a2 = make_adder(5);
    let r1 = a1(6);
    let r2 = a2(6);
    println!("r1(6) is {r1}");
    println!("r2(6) is {r2}");
}
