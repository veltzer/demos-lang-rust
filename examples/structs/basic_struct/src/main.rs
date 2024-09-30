// This is a simple example of a structs
// structs has methods by not inheritance.
// Notes:
// * note the comma on the last line, allowed and recommended
// * no semi-colon at the end of a struct

#[allow(dead_code)]

#[derive(Debug)]
struct Data {
    some_bool: bool,
    some_float: f64,
    some_int: i32,
}

fn main() {
    let var = Data {
        some_bool: true,
        some_float: 10.3,
        some_int: 80,
    };
    println!("{:?}", var);
}
