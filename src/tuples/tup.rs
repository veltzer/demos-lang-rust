fn main() {
    let mut x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let mut one = x.2;
    
    //one = one + 1;

    x.2 = x.2 + 1;
}
