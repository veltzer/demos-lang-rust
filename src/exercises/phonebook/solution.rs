// This is a solution to the phonebook exercise

use std::collections::HashMap;
use std::io::{stdout,stdin,Write,BufReader,BufRead};
use std::fs::File;

const FILENAME: &str="phonebook.txt";

fn read_book() -> HashMap::<String, String> {
	let mut pb = HashMap::<String, String>::new();
    let file = File::open(FILENAME).unwrap();
    let lines = BufReader::new(file).lines(); 
    // pb.insert("mark".to_string(), "0505665636".to_string());
    for line in lines {
        let fline = line.unwrap();
        // let parts: Vec<&str> = fline.strip_suffix("\n").unwrap().split(",").collect();
        let parts: Vec<&str> = fline.split(",").collect();
        println!("{:?}", parts);
        pb.insert(parts[0].to_string(), parts[1].to_string());
    }
    pb
}

fn print(pb: &HashMap<String, String>) {
	for (name, phone) in pb {
		println!("{name}: {phone}");
	}
}

fn write(pb: &HashMap<String, String>) {
    println!("in write...");
    let mut file = File::create(FILENAME).unwrap();
	for (name, phone) in pb {
		writeln!(&mut file, "{name},{phone}").unwrap();
    }
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

fn search(pb: &HashMap<String, String>) {
    println!("in search...");
    print!("enter name: ");
    stdout().flush().unwrap();
    let mut name: String = String::new();
    stdin().read_line(&mut name).unwrap();
    name = name.strip_suffix("\n").unwrap().to_string();
    println!("Phone is {:?}", pb.get(&name));
}

fn remove(pb: &mut HashMap<String, String>) {
    println!("in remove...");
    print!("enter name: ");
    stdout().flush().unwrap();
    let mut name: String = String::new();
    stdin().read_line(&mut name).unwrap();
    name = name.strip_suffix("\n").unwrap().to_string();
    pb.remove(&name);
}

fn add(pb: &mut HashMap<String, String>) {
    println!("in add...");
    print!("enter name: ");
    stdout().flush().unwrap();
    let mut name: String = String::new();
    stdin().read_line(&mut name).unwrap();
    name = name.strip_suffix("\n").unwrap().to_string();
    print!("enter phone: ");
    stdout().flush().unwrap();
    let mut phone: String = String::new();
    stdin().read_line(&mut phone).unwrap();
    phone = phone.strip_suffix("\n").unwrap().to_string();
    pb.insert(name, phone);
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
