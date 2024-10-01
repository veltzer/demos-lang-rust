fn main() {
    let r1: i32=5;
    let r: &i32;
    {
        let r2: i32=7;
        r = two_params(&r1, &r2);
        println!("r is {r}");
    }
    // this will cause a compilation error
    // println!("r is {r}");
}

fn two_params<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32 {
    if p1 > p2 {
        p1
    } else {
        p2
    }
}
