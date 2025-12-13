fn main() {
    //This is a string literal. It has a fixed size and is put in the stack
    // It cannot be modified let mut s = "hello" gives an error
    let s = "hello";
    println!("{}", s);

    //This is a String type. It is growable and mutable. It is stored in the heap
    let mut string1 = String::from("hello");
    string1.push_str(", world!"); // push_str appends a string slice to a String
    println!("{}", string1);

    //Ownership rules
    //1. Each value in Rust has a variable that is its owner
    //2. There can only be one owner at a time
    //3. When the owner goes out of scope, the value will be dropped

    {
        let s2 = String::from("hello"); // s2 is the owner of the string
        println!("{}", s2);
    } // s2 goes out of scope and the string is dropped here

    let s3 = String::from("hello");
    take_ownership(s3); // s3's value moves into the function and is no longer valid here
    // println!("{}", s3); // This would give an error because s3 is no longer valid

    let s4 = String::from("hello");
    let len = calculate_length(&s4); // We pass a reference to s4
    println!("The length of '{}' is {}.", s4, len); // s4 is still valid here

    // Copying vs Moving
    let x = 5;
    let y = x; // x is copied into y because i32 implements the Copy trait
    println!("x = {}, y = {}", x, y); // both x and y are valid here

    let s5 = String::from("hello");
    let s6 = s5; // s5 is moved to s6, s5 is no longer valid
    // println!("{}", s5); // This would give an error because s5 is no longer valid
    println!("s6 = {}", s6); // s6 is valid here

    // To clone a String, we can use the clone method
    let s7 = String::from("hello");
    let s8 = s7.clone(); // s7 is cloned to s8, both are valid
    println!("s7 = {}, s8 = {}", s7, s8);

    // These data types implement the Copy trait, so they are copied rather than moved
    // Examples: integers, booleans, floating-point numbers, characters,
    // tuples (if they only contain types that implement Copy)

    // Return values and scope

    s9 = gives_ownership(); // gives_ownership moves its return value to s9
    println!("s9 = {}", s9);
    let s10 = String::from("hello");
    let s11 = takes_and_gives_back(s10); // s10 is moved into the function and then moved out to s11
    // println!("{}", s10); // This would give an error because s10 is no longer valid
    println!("s11 = {}", s11);

    // Mutable references
    let mut s12 = String::from("hello");
    change(&mut s12); // We pass a mutable reference to s12
    println!("s12 = {}", s12); // s12 is modified by the function
    // If we said let s12 = String::from("hello"), we would get an error because s12 would be immutable

    // Summary:
    // Functions can take ownership of variables.
    // You can lose ownership when assigning a variable to another variable if they do not implement the Copy trait.
    // If your function is fn func(s: String), the function takes ownership of the argument.
    // To let a function use a variable without taking ownership, you can pass a reference using &:
    // fn func(s: &String)
    // To let a function modify a variable without taking ownership, you can pass a mutable reference using &mut:
    // fn func(s: &mut String)
    // Functions can also return ownership of a variable to the caller.
    // e.g fn func() -> String, the function gives ownership of the returned String to the caller.
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn take_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and 'drop' is called. The backing memory is freed

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but because it does not have ownership of what it refers to, nothing happens.
// This means we can still use s4 in the main function after passing a reference to this function.

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // some_string is returned and moves out to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the caller
}