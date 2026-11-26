// This example shows that if you want to disable a certain warning and want it for
// all functions and are using #[expect (or #[allow) you must do it for every function

#[expect(unused_variables, reason = "teaching: the attribute covers this function only")]
#[cfg_attr(test, expect(clippy::missing_const_for_fn, reason = "built as a test, main is an ordinary function that could be const"))]
fn main() {
    let x=42;
    other();
}

#[expect(unused_variables, reason = "teaching: every function needs its own attribute")]
const fn other() {
    let y=42;
}
