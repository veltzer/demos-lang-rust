use std::fs;

fn main() {
    let contents = fs::read_to_string("/etc/passwd").unwrap();
    print!("{contents}");
}
