
// This function copies the value of x into the function and returns it
fn another_function(x: i32)-> i32 {
    println!("The value of x is: {}", x);
    x
}

// This function takes a reference to x and returns it
fn another_function_by_reference(x: &i32) -> i32{
    println!("The value of x is: {}", *x); // Print the reference
    *x // Dereference the reference to get the value of x
}

//This function takes a mutable reference to x and modifies it. It returns its value
fn another_function_modify(x: &mut i32) -> i32 {
    *x += 1;
    println!("The value of x after modification is: {}", *x);
    *x
}

fn another_function_modify_with_var(x: &mut i32) -> i32 {
    let mut temp = *x;
    println!("The value of temp = *x before modification is: {}", temp);
    temp += 1;
    println!("The value of temp = *x after modification is: {}", temp);
    temp
}

fn another_function_modify_with_var_string(x: &mut String) -> String {
    let mut temp = x.clone();
    println!("The value of temp before modification is: {}", temp);
    temp.push_str(", world");
    println!("The value of temp after modification is: {}", temp);
    temp
}

fn main() {
    println!("let mut x = 5;");
    let mut x = 5;
    println!("The value of x is: {}", x); // 5
    let y = another_function(x); // 5
    println!("The return value of y is: {}", y); // 5
    let z = another_function_by_reference(&x);
    println!("The return value of z is: {}", z); // 5
    println!("The value of x is: {}", x); // 5
    let mut w = another_function_modify(&mut x);
    println!("The return value of w is: {}", w); // 6 
    println!("The value of x after modification is {}", x); // 6
    let v = another_function_modify_with_var(&mut x);
    println!("The return value of v is: {}", v); // 7
    println!("The value of x after another modification is still: {}", x); // 6
    // x was not modified because let mut temp = *x created a copy of x in temp, and modified the copy, not the original x
    // To modify x directly, we would need to dereference the mutable reference and modify it directly
    // Because of the copy trait for i32, the value was copied into temp, not moved
    // If we do this with a String, it would not work because String does not implement the Copy trait, and 
    // the *x derefence cannot be used to create a copy. Instead, we can clone it to create a copy.

    let mut x = String::from("Hello");
    println!("\nlet mut x = String::from(\"Hello\");");
    println!("The value of x is: {}", x); // Hello
    let y = another_function_modify_with_var_string(&mut x);
    println!("The return value of y is: {}", y); // Hello, world
    println!("The value of x after modification is still: {}", x); // Hello

    println!("\nUsing a code block to assign a value to y:");
    let y = {
        let x = 3;
        println!("The value of x in the code block is: {}", x);
        x + 1
    };
    println!("\nThe value of y is: {}", y);
    println!("The value of x outside the code block is: {}", x);

}
