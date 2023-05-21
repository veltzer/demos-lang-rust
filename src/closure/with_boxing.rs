fn make_adder(outer: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |inner: i32| outer+inner)
}

fn main() {
    let a1 = make_adder(3);
    let a2 = make_adder(5);
    let r1 = a1(6);
    let r2 = a2(6);
    println!("r1(6) is {r1}");
    println!("r2(6) is {r2}");
}
