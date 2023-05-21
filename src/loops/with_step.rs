// This is an example of who to do a for loop with step size != 1
// References:
// - https://stackoverflow.com/questions/27893223/how-do-i-iterate-over-a-range-with-a-custom-step

fn main() {
    for n in (0..40).step_by(5)  {
        println!("n is {n}");
    }
}
