use rand::Rng;

fn random_choose<'a>(option1: &'a String, option2: &'a String) -> &'a String {
    if rand::rng().random_bool(0.5) { option1 } else { option2 }
}

// Function 2: Creates Strings, calls random_choose, and prints
fn main() {
    let first: String = String::from("coffee");
    let chosen;
    // let mut chosen: String = String::from("");
    {
        let second = String::from("tea");
        chosen = random_choose(&first, &second);
        println!("chosen: {}", chosen);
    }
    // next line will cause a compilation error
    // println!("chosen: {}", chosen);
}
