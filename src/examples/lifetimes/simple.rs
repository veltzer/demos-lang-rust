/*
 * This example shows that you cannot use a slice after the string it
 * is connected too has been mutated or dropped.
 */

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    s.push_str("!"); // so the compiler won't comlpain that s is mut
    let word = first_word(&s);
    println!("the first word is: {}", word);
    // any of the following three options will cause "word" to be un-usable
    //s.clear();
    //s.push_str("m");
    //std:: mem::drop(s);
    println!("the first word is: {}", word);
}
