// This is an example of how to copy fields from one struct to another.

#[derive(Debug)]
struct Data {
    some_bool: bool,
    some_float: f64,
    some_int: i32,
    some_string: String,
}

fn main() {
    let var = Data {
        some_bool: true,
        some_float: 10.3,
        some_int: 80,
        some_string: "foo".into(),
    };
    let var2 = Data {
        some_int: 200,
        ..var
    };
    // println!("{:?}", var.some_string);
    println!("{:?}", var2);
    println!("{} {} {} {}", var2.some_bool, var2.some_float, var2.some_int, var2.some_string);
}
