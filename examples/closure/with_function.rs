// This is again an example of a clsure but this time the magic is more obvious.
// Ask yourself: where is the 3 and the 5 after the calls to "make_adder" are over?
// Were they not on the stack? If so, were they not removed?
// How come they still exist somewhere?

fn make_adder(outer: i32) -> impl Fn(i32) -> i32 {
    move |inner: i32| outer+inner
}

fn main() {
    let a1 = make_adder(3);
    let a2 = make_adder(5);
    let r1 = a1(6);
    let r2 = a2(6);
    println!("r1(6) is {r1}");
    println!("r2(6) is {r2}");
}
