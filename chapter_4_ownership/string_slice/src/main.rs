

fn main() {
    // This is a string literal, which is a slice pointing to a fixed string in the binary.
    let s: &str = "Hello, world!";
    println!("{}", s);

    //This is a String, which is a heap-allocated string.
    let mut string_obj = String::from("Hello, ");
    string_obj.push_str("world!");
    println!("{}", string_obj);

    // Slicing a String to get a string slice
    let hello_slice: &str = &string_obj[0..5];
    println!("{}", hello_slice);

    // Demonstrating that string slices are UTF-8 aware
    let unicode_string = String::from("Здравствуйте");
    let unicode_slice: &str = &unicode_string[0..4]; // Slicing the first two characters
    println!("{}", unicode_slice);

    // Finding the first word in a string
    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("The first word is: {}", word);

    // String slices can also be used with string literals
    let my_literal = "hello world";
    let my_literal_string = my_literal.to_string();
    let word_literal = first_word(&my_literal_string);
    println!("The first word in the literal is: {}", word_literal);

    // Demonstrating string slice syntax variations
    let full_slice: &str = &string_obj[..];
    let from_start_slice: &str = &string_obj[0..];
    let to_end_slice: &str = &string_obj[..string_obj.len()];
    println!("Full slice: {}", full_slice);
    println!("From start slice: {}", from_start_slice);
    println!("To end slice: {}", to_end_slice);

    // Using string slices with functions
    let another_string = String::from("Goodbye world");
    let first_word_in_another = first_word(&another_string);
    println!("The first word in another string is: {}", first_word_in_another);

    //It is better to use string slices as function parameters to accept both String and &str types
    //This is because &str can accept string literals as well as String objects.
    let first_word_in_literal = first_word_literal("Hello everyone");
    println!("The first word in the literal is: {}", first_word_in_literal);
    let string_obj = String::from("Hello everyone");
    let first_word_in_string = first_word_literal(&string_obj);
    println!("The first word in the string object is: {}", first_word_in_string);

}

fn first_word_literal(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
