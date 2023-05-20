// this example prints out various simple types of rust
use std::io::stdout;
use std::io::*;
use std::fs::File;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let x: i32=7;
    print_type_of(&x);
    print_type_of(&stdout());
    let file = File::open("/etc/passwd").unwrap();
    print_type_of(&file);
    let lines = BufReader::new(file).lines(); 
    print_type_of(&lines);
}
