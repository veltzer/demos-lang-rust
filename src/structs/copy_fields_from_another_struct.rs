// This is an example of how to copy fields from one struct to another.

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
    let var2 = Data {
        some_int: 200,
        ..var
    };
    println!("{:?}", var2);
}
