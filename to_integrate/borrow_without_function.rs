fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(" world");
    {
        let s2:&mut String=&mut s1;
        println!("s2 is {s2}");
    }
    println!("s1 is {s1}");
    // let len = calculate_length(&mut s1);
    // println!("The length of '{}' is {}.", s1, len);
}

/*
fn calculate_length(s: &mut String) -> usize {
    s.push_str("!");
    s.len()
}
*/
