// This example shows that either all elements of a tuple are mutable or
// they are not. There is no way to make just one element of a tuple mutable.

fn main() {
    let mut x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let mut _one = x.2;
    _one = _one + 1;
    x.2 = x.2 + 1;
}
