// This is an example of a closure without functions at all
// At first you may not understand what the fuss is all about...

fn main() {
    let mut m=5;
    let c1 = |i| { i+5+m };
    let c2 = |i| { i+3+m };
    let r1 = c1(6);
    let r2 = c2(6);
    m=6;
    println!("c1(6) is {r1}");
    println!("c2(6) is {r2}");
    println!("c1(6) is {r1}");
    println!("c2(6) is {r2}");
    println!("m is {m}");
}
