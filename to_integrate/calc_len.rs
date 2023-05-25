fn main() {
    print!("Give me a string: ");
    let mut name: String = String::new();
    let len = calculate_length(&mut name);
    println!("The length of {name} is {len}");
}

fn calculate_length(s: &mut String) -> usize {
    s.len()
}
