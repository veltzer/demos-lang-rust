use std::os::raw::c_int;
use std::os::raw::c_char;
use std::ffi::CString;

#[repr(C)]
pub struct my_struct {
    pub x: c_int,
    pub y: c_int,
}

// #[link(name = "clib")]
extern "C" {
    fn add(a: c_int, b: c_int) -> c_int;
    fn print_string(s: *const c_char);
    //fn crash_me();
    fn get_struct_from_rust(p: *mut my_struct);
}

fn main() {
    let result = unsafe { add(5, 7) };
    println!("5 + 7 = {}", result);

    let rust_string = "Hello from Rust!";
    let c_string = CString::new(rust_string).unwrap();

    unsafe {
        print_string(c_string.as_ptr());
    }

    let mut p = my_struct {
        x: 7,
        y: -9,
    };
    unsafe {
        get_struct_from_rust(&mut p as * mut my_struct);
    }
    println!("{}", p.x);
    println!("{}", p.y);


    /*
    unsafe {
        crash_me();
    }
    */
}
