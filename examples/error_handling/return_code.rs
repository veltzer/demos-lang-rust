use std::fs::File;
//use std::error::Error;

fn main() {
// fn main() -> Result<(), Box<dyn Error>> {
    //let _greeting_file = File::open("hello.txt")?;
    let _greeting_file = File::open("hello.txt").unwrap();
    //Ok(())
}
