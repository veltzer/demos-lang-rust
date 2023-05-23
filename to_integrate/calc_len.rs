fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(" world");
    let len = calculate_length(&mut s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &mut String) -> usize {
    // i dont know how to calculate length so I will the real one
    real_calc(s)
}

fn real_calc(s: &mut String) -> usize {
    s.push_str("!");
    let _x=s;
    let z: usize = s.len();
    z
}
