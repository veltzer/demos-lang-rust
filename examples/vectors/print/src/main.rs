// An example of how to print vectors
//
// References:
// - https://doc.rust-lang.org/std/vec/struct.Vec.html

fn main() {
    let v = vec![1, 3, 5];
    println!("{:?}", v);
    for x in &v {
        println!("{x}");
    }
}
