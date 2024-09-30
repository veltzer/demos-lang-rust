// This example shows that if you want to disable a certain warning and want it for
// all functions and are using the #[allow you must do it for every function

#[allow(unused_variables)]
fn main() {
    let x=42;
    other()
}

#[allow(unused_variables)]
fn other() {
    let y=42;
}
