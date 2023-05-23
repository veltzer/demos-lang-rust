fn take_it_and_give_it_back(_s: String) -> String {
    //s
    String::from("goodbye")
}


fn main() {
    let mut s1 = String::from("hello");
    s1 = take_it_and_give_it_back(s1);
    println!("{}, world!", s1);
}
