/// We've seen how wrapping data types and structs in wrappers can be useful
/// We can use this to enforce type safety by using methods for specific wrappers of types
/// but not on the types themselves
/// 
/// Type aliases are useful for wrapping types to make them more readable

type Thunk = Box<dyn Fn() + Send + Sync>; 
// This is a type alias for a boxed function that can be sent and synced
fn takes_long_type(f: Thunk) {
    f();
}

/// Most commonly it is used for Results
/// Because a Result<T,E> typically uses an error Enum for E, say std::io::Error
/// we can just create a type alias to write cleaner code
/// We can use type aliasing in conjunction with generic types

type Result<T> = std::result::Result<T,std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> Result<()>;
}

/// Never types are denoted as !, and we can use them to create diverging functions
/// Primarily this is useful for creating match arms of differing types.
/// Because they do not return anything, no return type can be inferred.
fn foo() -> ! {
    panic!("blah")
}
// Below we will not use foo(), but instead break/continue can be used to manipulate a loop
// Most commonly diverging functions are used to exit process


/// A generic function must know the size of a type T at compile time.
/// This implicitly creates a T: Sized requirement
/// fn generic<T> (t: T) { ... }
/// To relax this constraint we use ?Sized
/// But we still can only take arguments by reference, as borrowing the value requires both Sized and Copy
/// To be implemented
/// Where possible, Rust will automatically implement Sized
/// But this is why we do not make functions of str types, as these are DSTs (Dynamically Sized Types)
/// We always work with pointers to a string

fn generic<T: ?Sized> (t: &T){
    println!("blah");
}

fn main() {
    
    let f: Thunk = Box::new(|| println!("Hi"));
    takes_long_type(f);

    let mut x = 3;
    loop {
        match x {
            0 => break,
            x => println!("{}",x),
        }
        x-=1;
    }

}
