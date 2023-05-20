// this example prints out various simple types of rust

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let x: i32=7;
    print_type_of(&x);
}
