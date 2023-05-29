fn main() {
    let a;
    {
        let b: i32=5;
        a=&b;
    }
    println!("a is {a}");
}
