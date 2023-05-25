// An example of how to use vectors as stacks
//
// References:
// - https://doc.rust-lang.org/std/vec/struct.Vec.html

fn main() {
    let mut v = Vec::<i32>::new();
    v.push(1);
    v.push(3);
    v.push(5);
    while let Some(top) = v.pop() {
        println!("{top}");
    }
}
