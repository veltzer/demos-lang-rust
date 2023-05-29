#[derive(Debug)]
struct MyStruct<'a> {
    a: &'a i32,
    b: &'a i32,
}

fn main() {
    let i1: i32=5;
    let m: MyStruct;
    {
        // println!("{:?}", m);
        let i2: i32=7;
        m=MyStruct {
            a: &i1,
            b: &i2,
        };
        println!("{:?}", m);
        println!("{:?}", m.a);
        println!("{:?}", m.b);
    }
    my_func(&m);
    // println!("{:?}", m);
}

#[allow(dead_code)]
fn my_func(_m: &MyStruct) {
    // println!("{:?}", m.a);
}
