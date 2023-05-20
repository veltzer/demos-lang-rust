use std::fs;

fn main() {
    let contents = fs::read_to_string("/etc/passwd").expect("Should have been able to read the file");
    print!("{contents}");
}
