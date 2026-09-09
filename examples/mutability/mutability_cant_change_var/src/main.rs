// Declaring x first and assigning it later is what this example is about,
// so clippy's suggestion to merge the two lines would remove the lesson.
#[allow(clippy::needless_late_init)]
fn main() {
    let x;
    println!("hello");
    //println!("The value of x is: {x}");
    x=5;
    println!("The value of x is: {x}");
    //x = 6;
    //println!("The value of x is: {x}");
}
