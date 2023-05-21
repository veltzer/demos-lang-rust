// This is an example of how to time code using the
// std::time::Instant standard API. 
//
// References:
// - https://doc.rust-lang.org/std/time/struct.Instant.html

use std::time::Instant;

fn time_it(func: fn()) {
    let now = Instant::now();

    // Code block to measure.
    func();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn code() {
    let mut sum: i64=0;
    let mut add: bool=true;
    for i in 0..9999999999i64 { 
        if add {
            sum+=i*i;
        } else {
            sum-=i*i;
        }
        add = !add;
    }
    println!("sum is {sum}")
}

fn main() {
    time_it(code)
}
