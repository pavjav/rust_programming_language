pub trait HelloMacro {
    fn hello_macro();
}

extern crate macros_proc_macros;
use macros_proc_macros::route;

// This is an attribute macro
#[route(GET, "/")]
pub fn index() {
    println!("hello");
}

