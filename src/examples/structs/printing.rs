// This is an example of how to print structs
// Note that you need to derive from the "Debug" trait
// to get this capability
//
// References:
// - https://stackoverflow.com/questions/30253422/how-to-print-structs-and-arrays

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
