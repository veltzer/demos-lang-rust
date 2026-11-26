// This is a simple example of a structs
// structs has methods by not inheritance.
// Notes:
// * note the comma on the last line, allowed and recommended
// * no semi-colon at the end of a struct

#[derive(Debug)]
#[expect(clippy::struct_field_names, reason = "teaching: each field is named for the type it holds")]
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
    println!("{var:?}");
    println!("{} {} {}", var.some_bool, var.some_float, var.some_int);
}
