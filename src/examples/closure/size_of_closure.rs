pub fn make_adder(outer: i32) -> impl Fn(i32) -> i32 {
    let x=[1,2,3, 4];
    move |inner: i32| outer+inner+x[3]
}

pub fn main() {
    let a1 = make_adder(3);
    let a2 = make_adder(5);
    println!("Size of a1: {} bytes", std::mem::size_of_val(&a1));
    let r1 = a1(6);
    let r2 = a2(6);
    println!("r1(6) is {r1}");
    println!("r2(6) is {r2}");
}
