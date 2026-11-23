// This example shows that if you want to disable a certain warning and want it for
// all functions and are using #[expect (or #[allow) you must do it for every function

#[expect(unused_variables, reason = "teaching: the attribute covers this function only")]
fn main() {
    let x=42;
    other()
}

#[expect(unused_variables, reason = "teaching: every function needs its own attribute")]
fn other() {
    let y=42;
}
