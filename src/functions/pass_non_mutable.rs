use std::io::{stdout,stdin,Write};

fn main() {
    print!("Give me a string: ");
    stdout().flush().unwrap();
    let mut s: String = String::new();
    stdin().read_line(&mut s).unwrap();
    s = s.strip_suffix("\n").unwrap().to_string();
    let len = calculate_length(&s);
    println!("The length of {s} is {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
