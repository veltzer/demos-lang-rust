// This is an example of what happens when you get an error value and try to continue,
// for exapmle using unwrap().
// In this case the code returned to the OS/parent is 101

use std::fs::File;

fn main() {
    let _greeting_file_result = File::open("there_is_no_such_file").expect("cannot open file");
}
