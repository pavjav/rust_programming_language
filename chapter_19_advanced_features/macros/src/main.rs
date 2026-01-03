/// Macros are used for metaprogramming.
/// Macros like println! and vec! are used extensively to reduce the amount of code written
/// here is a simple implementation of vec

#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// Macros can be very difficult to write, but give way more flexibility to the user
/// Check out Little Book of Rust Macros for more
/// 
/// 
/// derive() macros are for implementing traits automatically
/// We have to use proc macros for derive macros and they have to be in their own crate.
/// We create a crate called macros-proc-macros in macros
/// We add this to that crate's Cargo.toml
/// [dependencies]
/// syn = "1.0"
/// quote = "1.0"
///
/// [lib]
/// proc-macro = true
/// 
/// Within this crate we just add a dependency on the proc-macros library
/// 
/// [dependencies]
/// hello_macro_derive = { path = "../macros/macros-proc-macros"}
/// 
/// We will define this macro in macros/macros-proc-macros/lib.rs

use macros::HelloMacro;
use macros_proc_macros::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;



fn main() {
    let v= my_vec![1,2,3];
    println!("{:?}", v);
    Pancakes::hello_macro();
    macros::index(); // At compile time, the macro on this function will run

    macros_proc_macros::sql!("SELECT * FROM FOO WHERE BAR"); // At compile time this will run
}
