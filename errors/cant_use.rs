fn do_something_with_my_string(some_string: String) {
    println!("{some_string}, world!");
}


fn main() {
    let s = String::from("hello");
    do_something_with_my_string(s);
    println!("{s}, world!");
}
