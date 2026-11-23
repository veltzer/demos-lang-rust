// This example shows that putting the attribute on an enclosing module
// disregards the warning for every function in that module.

#[expect(unused_variables, reason = "teaching: one attribute on the module covers all its functions")]
mod demo {
    pub fn first() {
        let x=42;
        second()
    }

    fn second() {
        let y=42;
    }
}

fn main() {
    demo::first();
}
