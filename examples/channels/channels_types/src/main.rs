// This demos is intended to find out the types of channels in rust
use std::sync::mpsc;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    let c = mpsc::channel::<i32>();
    print_type_of(&tx);
    print_type_of(&rx);
    print_type_of(&c);
}
