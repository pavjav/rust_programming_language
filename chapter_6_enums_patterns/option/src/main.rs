//Option is essentially a templated enum
/*
enum Option<T> {
    None,
    Some(T)
}

*/

fn main() {
    //
    let some_number = Some(5); //Option<i32>
    let some_char = Some('e'); //Option<char>
    let absent_number = None; //Option<T>
    let absent_char = None;

    //When handling option types, we'll need to use unwrap_or()
    //WARNING unwrap_or() will take ownership if the type does not have a Copy trait
    println!("some_number is {}", some_number.unwrap_or(0));
    println!("some_char is {}", some_char.unwrap_or('\0'));
    println!("absent_number is {}", absent_number.unwrap_or(0));
    println!("absent_char is {}", absent_char.unwrap_or('\0'));
    //Because T is int32, we can re-use it.
    println!("some_number is {}", some_number.unwrap_or(0));

    // When we create an Option<T> type for T that does not have a Copy trait, things start breaking
    // We must start being explicit about Option types when assigning None
    let some_string = Some(String::from("Hello"));
    let absent_string: Option<String> = None;
    println!("some_string is {}", some_string.unwrap_or(String::from("")));
    println!("absent_string is {}", absent_string.unwrap_or(String::from("")));
    // If we rerun this, unwrap_or takes ownership of some_string and this no longer works
    //println!("some_string is {}", some_string.unwrap_or(String::from("")));
    // To fix this, we must instead use the string's clone method:
    let some_string_reuse = Some(String::from("Hello"));
    println!("some_string_reuse is {}",some_string_reuse.clone().unwrap_or(String::from("")));
    println!("some_string_reuse is {}",some_string_reuse.clone().unwrap_or(String::from("")));

    //Or we can use the as_deref() method to convert to a string literal:
    let some_string_reuse_deref = Some(String::from("Hello"));
    println!("some_string_reuse_deref is {}",some_string_reuse_deref.as_deref().unwrap_or(""));
    println!("some_string_reuse_deref is {}",some_string_reuse_deref.as_deref().unwrap_or(""));

    

    // We cannot add Option<i32> to i32, e.g. some_number + 1 will give an error.
    // We can instead using matching with Option<T>
    // We can also use the unwrap_or_default() method for standard conversion behavior (numeric is 0, string is "", etc)
    let increment = plus_one(&some_number);
    println!("increment is {}", increment.unwrap_or_default());
    println!("Original number was: {}", some_number.unwrap_or_default());
    let absent_increment = plus_one(&absent_number);
    println!("absent_increment is {}", absent_increment.unwrap_or_default());
    println!("Original number was: {}", absent_number.unwrap_or_default());

    // We can use _ as a catchall in a match block
    let x = catch_all(&Some(1));
    let y = catch_all(&Some(2));
    let z = catch_all(&None);

    println!("Some(1) -> {}, Some(2) -> {}, None -> {}", x.unwrap_or_default(), y.unwrap_or_default(), z.unwrap_or_default());

    // Mutating Options (or ENUMs in general) can get complicated.
    // create mutable Option<i32>
    let mut input: Option<i32> = Some(1);
    println!("input was {}",input.unwrap_or_default());
    //Pass by reference to a function
    modify_option(&mut input);

    println!("input is now {}", input.unwrap_or_default());

    let mut null_input: Option<i32> = None;
    println!("input was {}", null_input.unwrap_or_default());
    modify_option(&mut null_input);
    println!("input is now {}", null_input.unwrap_or_default());

}


fn plus_one(x: &Option<i32>) -> Option<i32> {
    // We take an optional x, and match depending on its type.
    // It takes  Some(i) to Some(i+1)
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn catch_all(x: &Option<i32>) -> Option<i32> {
    match x {
        Some(1) => Some(10),
        Some(2) => None,
        _ => Some(20)
    }
}

//This is an example of concise control flow with if let:
// We do this to make sure the optional value matches a pattern
// We also do this so that we can make mutate the data.
// The else block will map None->Some(100)
fn modify_option(x: &mut Option<i32>) {
    if let Some(i) = x{
        *i+=1;
    }
    else {
        *x = Some(100);
    }

    // This does not work. The above does a better job of mutating optional values

    //match x {
    //    Some(i) => Some(*i+1),
    //    None => None
    //};
}