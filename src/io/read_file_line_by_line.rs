use std::fs::File;

fn main() {
    let _file = File::open("/etc/passwd").unwrap();
}
