// This shows various ways to creaet a string
// Remember that strings are always on the heap

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn main() {
    let s: String = String::from("hello");
    println!("s is [{s}]");
    print_type_of(&s);
}
