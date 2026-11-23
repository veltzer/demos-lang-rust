// This is an example of an unused variabe.
// If you remove the attribute below you will get a compiler warning.
// another option is to call the variable _* which allows it to be unused (similar to python).

#[expect(unused_variables, reason = "teaching: the attribute silences the unused-variable warning")]
fn main() {
    let _x: i32=0;
    let x: i32=0;
    println!("Hello World!");
}
