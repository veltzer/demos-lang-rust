const MY_CONST: &str="foo";

#[warn(unused_variables)]
fn main() {
    my_func(MY_CONST);
}

fn my_func(_p: &str) {
}
