// An example of how to print vectors
//
// References:
// - https://doc.rust-lang.org/std/vec/struct.Vec.html

fn main() {
    let mut v = Vec::<i32>::new();
    v.push(1);
    v.push(3);
    v.push(5);
    println!("{:?}", v);
}
