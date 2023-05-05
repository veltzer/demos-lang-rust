extern crate num_cpus;

fn main() {
    println!("number of cores is {}", num_cpus::get());
}
