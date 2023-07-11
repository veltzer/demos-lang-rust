/*
 * In this example if you remove the explicit type given to _guess
 * the example stops compiling
 */

fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");
}
