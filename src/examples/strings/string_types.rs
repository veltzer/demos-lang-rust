// This example demonstrates the various string types in Rust.
// We compare the following:
// - real strings ("alloc::string::String")
// - slices/&str ("&str")
// - Box<str>
//
// Notes:
// Strings own their memory, slices don't.
// slices are just pointers, either to stack or to heap
// Boxes cost a little more than the thing they box

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // this is a regular string
    let string: String = String::from("hello");
    print_type_of(&string);
    println!("string is [{string}]");

    // this is a slice, or &str
    let slice: &str = "hello";
    print_type_of(&slice);
    println!("slice is [{slice}]");

    // you may think you can write the following line but you really cannot,
    // as it gives you a compilation error:
    // "doesn't have a size known at compile-time"
    // remember, in Rust stuff on the stack must be of known size.
    //let real_str: str = "hello";

    // a differrent way of doing this is using Box
    // Box allows you to store a pointer, which is of constant size, to something on the heap
    let boxed: Box<str> = "hello".into();
    print_type_of(&boxed);
    println!("boxed is [{boxed}]");
}
