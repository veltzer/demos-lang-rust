// This is a simple example of a HashMap in rust

use std::collections::HashMap;
use std::io::{stdout,stdin,Write,BufReader,BufRead};
use std::fs::File;

fn read_book() -> HashMap::<String, String> {
	let mut pb = HashMap::<String, String>::new();
    let file = File::open("phonebook.txt").unwrap();
    let lines = BufReader::new(file).lines(); 
    pb.insert("mark".to_string(), "0505665636".to_string());
    for _line in lines {
         // let parts: Vec<&str> = line.unwrap().strip_suffix("\n").unwrap().split(",").collect();
         //pb.insert(parts[0].to_string(), parts[1].to_string());
    }
    pb
}

fn print(pb: &HashMap<String, String>) {
	for (name, phone) in pb {
		println!("{name}: {phone}");
	}
}

fn write(_pb: &HashMap<String, String>) {
}

fn print_menu() -> i32 {
    println!("menu");
    println!("1) print book");
    println!("2) search for name");
    println!("3) remove name");
    println!("4) add name");
    println!("5) write book");
    println!("6) exit");
    print!("Selection => ");
    stdout().flush().unwrap();
    let mut selection: String = String::new();
    stdin().read_line(&mut selection).unwrap();
    selection.strip_suffix("\n").unwrap().parse().unwrap()
}

fn search(_pb: &HashMap<String, String>) {
    println!("in search...");
}

fn remove(_pb: &mut HashMap<String, String>) {
    println!("in remove...");
}

fn add(_pb: &mut HashMap<String, String>) {
    println!("in add...");
}

fn main() {
    let mut phonebook = read_book();
    loop {
        let selection = print_menu();
        match selection {
            1=> print(&phonebook),
            2=> search(&phonebook),
            3=> remove(&mut phonebook),
            4=> add(&mut phonebook),
            5=> write(&phonebook),
            6=> break,
            _ => println!("didn't get you..."),
        }
    };
}
