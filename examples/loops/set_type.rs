// This is an example of how to set the type of the integer for range loops
// References:
// - https://stackoverflow.com/questions/59663036/allow-i-to-be-u64-in-a-for-loop-over-range

fn main() {
    for n in 0..10u32 {
        println!("n is {n}");
    }
}
