// This example demonstrates the difference between "String"
// and "StringSlice"
// Strings own their memory, slices don't.
// slices are just pointers, either to stack or to heap

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let string: String = String::new();
    let slice: &str = "Hello";
    print_type_of(&string);
    print_type_of(&slice);
}
