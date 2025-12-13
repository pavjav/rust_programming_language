fn main() {
    let mut counter = 0;
    
    // Loop that increments counter until it reaches 5
    loop {
        counter += 1;
        println!("Counter: {}", counter);
        if counter >= 5 {
            break;
        }
    };

    let mut counter = 0;
    // Loop with a return value
    let result = loop {
        counter += 1;
        println!("Counter in result loop: {}", counter);
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    // Loop labels example allows breaking out of nested loops
    let mut count = 10;
    'outer: loop {
        println!("Outer Count: {}", count);
        let mut inner_count = 10;
        'inner: loop {
            println!("Inner count: {}", inner_count);
            if inner_count == 5 {
                break 'inner;
            }
            if count == 2 {
                break 'outer;
            }
            inner_count -= 1;
        }
        count -= 1;
    }

    // While loop example
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // For loop example iterating over an array
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }
    // For loop with a range and rev to count down
    for number in (1..4).rev() {
        println!("{}!", number);
    // For loop via indexing
    }    for i in 0..a.len() {
        println!("The value via indexing is: {}", a[i]);
    }
    println!("LIFTOFF!!!");
    
}
