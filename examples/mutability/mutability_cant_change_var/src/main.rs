// Declaring x first and assigning it later is what this example is about,
// so clippy's suggestion to merge the two lines would remove the lesson.
#[expect(clippy::needless_late_init, reason = "teaching: declaring x first and assigning later is the point")]
fn main() {
    let x;
    println!("hello");
    //println!("The value of x is: {x}");
    x=5;
    println!("The value of x is: {x}");
    //x = 6;
    //println!("The value of x is: {x}");
}
