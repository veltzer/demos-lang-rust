use std::fs::File;
use std::io::*;

/*
 * This is a solution to the exercise.
 */

fn main() {
    let mut shell = String::new();
    stdin().read_line(&mut shell).expect("Failed to read line");
    shell = shell.strip_suffix("\n").unwrap().to_string();
    // println!("shell is [{shell}]");
    let file = File::open("/etc/passwd").unwrap();
    let lines = BufReader::new(file).lines(); 
    for line in lines {
        let fline = line.unwrap();
        let parts: Vec<&str> = fline.split(":").collect();
        let current_shell = parts[6];
        if current_shell==shell {
            let username = parts[0];
            println!("{username}");
        }
    }
}
