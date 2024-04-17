use std::process;

fn main() {    
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s = choose_one_of_two(s1, s2);
    println!("{}", s);
}

fn choose_one_of_two(s1: String, s2: String) -> String {
    let my_pid = process::id();
    println!("my_pid = {}", my_pid);
    if my_pid % 2 == 0 {
        s1
    } else {
        s2
    }
}
