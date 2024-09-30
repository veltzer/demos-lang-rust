// This example shows that using the $![allow construct allows to disregard
// warnings for the whole file


#![allow(unused_variables)]

fn main() {
    let x=42;
    other()
}

fn other() {
    let y=42;
}
