// A struct that can hold both sized and unsized types
struct Container<T: ?Sized> {
    data: Box<T>,
}

impl<T: ?Sized> Container<T> {
    // Constructor for sized types
    fn new<U>(value: U) -> Container<U> {
        Container {
            data: Box::new(value),
        }
    }
}

fn main() {
    // With a sized type (i32)
    let container_int = Container::<i32>::new(42);
    
    // With an unsized type (str)
    let boxed_str = "Hello".to_string().into_boxed_str();
    let container_str = Container { data: boxed_str };
}
