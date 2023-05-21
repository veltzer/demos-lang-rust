#[allow(unused_variables)]

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let v1: Vec<_> = (1..10).collect();
    let v2: Vec<i32> = (1..10).collect();
    print_type_of(&v1);
    print_type_of(&v2);
}
