fn main() {
    let r1: i32=5;
    let mut r: &i32;
    {
        let r2: i32=7;
        two_params(&r1, &r2);
         println!("r is {r}");
    }
    // println!("r is {r}");
}

static mut g: &i32=&5;

fn two_params<'a, 'b>(p1: &'a i32, p2: &'a i32) {
    if p1 > p2 {
        g=p1;
    } else {
        g=p2;
    }
}
