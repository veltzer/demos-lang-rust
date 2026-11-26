// This is an example of how not to get an error from the compiler about an unused variable
// by prefixing it with an underscore.

fn main() {
    #[expect(clippy::no_effect_underscore_binding, reason = "teaching: the binding is there to be read, not used")]
    let _x: i32=0;
    println!("Hello World!");
}
