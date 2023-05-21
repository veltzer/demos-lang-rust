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

fn code1(limit: i64) {
    let mut sum: i64=0;
    let mut add: bool=true;
    for i in 0..limit { 
        if add {
            sum+=i;
        } else {
            sum-=i;
        }
        add = !add;
    }
    println!("sum is {sum}")
}

fn code2(limit: i64) {
    let mut sum: i64=0;
    for i in 0..limit { 
        sum+=i;
    }
    /*
    for i in (1..limit).step_by(2) { 
         sum-=2*i;
    }
    */
    println!("sum is {sum}")
}

fn main() {
    const LIMIT: i64=9999999999;
    time_it(|| { code1(LIMIT) });
    time_it(|| { code2(LIMIT) });
}
