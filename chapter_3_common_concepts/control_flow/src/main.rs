
//Pass function by reference and return a boolean value if x is divisible by 3 or 5
fn another_function(x: &i32) -> bool{
    if *x % 3 == 0 {
        true
    } else if *x % 5 == 0 {
        true
    } else {
        false
    }
}

fn main() {
    let x = 15;
    println!("The value of x is: {}", x);
    let is_divisible = another_function(&x);
    println!("Is x divisible by 3 or 5? {}", is_divisible);
    let y = 7;
    println!("The value of y is: {}", y);
    let is_divisible_y = another_function(&y);
    println!("Is y divisible by 3 or 5? {}", is_divisible_y);

    // Using if expression to assign a value to a variable
    let number: i32 = if is_divisible {
        10
    } else {
        20
    };
    println!("The value of number is: {}", number);

    let number: i32 = if is_divisible_y { 10 } else { 20 };
    println!("The value of number is: {}", number);
}
