// This example shows that putting the attribute on an enclosing module
// disregards the warning for every function in that module.

#[expect(unused_variables, reason = "teaching: one attribute on the module covers all its functions")]
mod demo {
    pub const fn first() {
        let x=42;
        second();
    }

    const fn second() {
        let y=42;
    }
}

#[cfg_attr(test, expect(clippy::missing_const_for_fn, reason = "built as a test, main is an ordinary function that could be const"))]
fn main() {
    demo::first();
}
