
// This function copies the value of x into the function and returns it
fn another_function(x: i32)-> i32 {
    println!("The value of x is: {}", x);
    x
}

// This function takes a reference to x and returns it
fn another_function_by_reference(x: &i32) -> i32{
    println!("The value of x is: {}", x);
    *x
}

//This function takes a mutable reference to x and modifies it. It returns its value
fn another_function_modify(x: &mut i32) -> i32 {
    *x += 1;
    println!("The value of x after modification is: {}", *x);
    *x
}

fn main() {
    println!("let mut x = 5;");
    let mut x = 5;
    println!("The value of x is: {}", x);
    let y = another_function(x);
    println!("The return value of y is: {}", y);
    let z = another_function_by_reference(&x);
    println!("The return value of z is: {}", z);
    println!("The value of x is: {}", x);
    let mut w = another_function_modify(&mut x);
    println!("The return value of w is: {}", w);
    println!("The value of x after modification is {}", x);
    w +=1;
    println!("The value of w after modification is: {}", w);
    println!("The value of x is still: {}", x);

    println!("\nUsing a code block to assign a value to y:");
    let y = {
        let x = 3;
        println!("The value of x in the code block is: {}", x);
        x + 1
    };
    println!("\nThe value of y is: {}", y);
    println!("The value of x outside the code block is: {}", x);

}
