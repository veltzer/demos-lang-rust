// This example will show you that many functions in rust return a Result type

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let s = String::new();
    print_type_of(&s);
}
