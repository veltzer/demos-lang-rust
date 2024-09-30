// this example shows that if you use integers in your code without
// any type names you will get i32

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let what_is_my_type=8;
    print_type_of(&what_is_my_type);
    for mistery in 1..10 {
        print_type_of(&mistery);
    }
    // next line will give you a compilation error because the number is too big for i32
    // note: the literal `10000000000000` does not fit into the type `i32` whose range is `-2147483648..=2147483647`
    // let what_am_i=10000000000000;
    let what_am_i: i64=10000000000000;
    print_type_of(&what_am_i);
}
