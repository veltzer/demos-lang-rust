// This is an example of taking and returning ownership of a string
// note that there is no obligation to return the same string that you were given

fn take_it_and_give_it_back(_s: String) -> String {
    String::from("goodbye")
    // s
}


fn main() {
    let mut s = String::from("hello");
    s = take_it_and_give_it_back(s);
    println!("{s}, world!");
}
