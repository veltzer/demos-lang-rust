// This is an example of borrowing an object without a function but rather
// with just a scope.

fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(" world");
    {
        let s2:&mut String=&mut s1;
        println!("s2 is {s2}");
    }
    println!("s1 is {s1}");
}
