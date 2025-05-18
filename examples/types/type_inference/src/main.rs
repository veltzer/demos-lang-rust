use std::collections::HashMap;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn main() {
    let mut scores = HashMap::new();
    print_type_of(&scores);
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 7.2);
}
