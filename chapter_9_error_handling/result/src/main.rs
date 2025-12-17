fn main() {
    // Error handling in Rust is facilitated by the Result enum:
    // enum Result<T,E> {
    //     Ok(T),
    //     Err(E)
    // }
    // Some libraries automatically use this
    // We can take a result and use panic! to throw an error
    use std::fs::File;

    let result = File::open("hello.txt");

    // This is commented out so rest of the program runs
    /*
    let file = match result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
            
        }
    };
    */

    // We can further use ErrorKinds to resolve errors
    // the error object has a .kind() method that returns an ErrorKind object
    // ErrorKind has the following enum values:
    // enum ErrorKind{
    //     NotFound,
    //     ...
    // }
    use std::io::ErrorKind;
    let result = File::open("test.txt");
    let file = match result { // First we match on the Result enum
        Ok(file) => file, 
        Err(error) => match error.kind() { // Then we match on ErrorKind enum
            ErrorKind::NotFound => {
                match File::create("test.txt") {
                    Ok(created_file) => created_file,
                    Err(e) => panic!(
                        "Problem creating the file {:?}",
                        e
                    )
                }
            },
            other_error => { // We can use anything for default behavior, commonly we use _ but here we will be descriptive
                panic!(
                    "Problem openin the file: {:?}",
                    other_error
                )
            }
            
        }
        
    };

    // A cleaner way to achieve the same thing:

    let file = File::open("test2.txt").unwrap_or_else(
        |error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("test2.txt"). unwrap_or_else(
                    |error| {
                        panic!("Problem creating the file: {:?}", error);
                    }
                )
            }
            else {
                panic!("Problem opening the file: {:?}", error);
            }
        }
    );

    /* This is an example of a closure. A closure is like a lambda function.
    .unwrap_or_else() takes an operation as an argument. If result is Ok, then it will "unwrap",
    essentially return the value type that is stored in Ok. Otherwise, it will take an op of type
    F: FnOnce(E) -> T + Destruct
    Basically F is a function of the error and has to implement the Destruct trait.
    Normally F should return something of type T, which in this case is a File
    But using the closure |error| {panic!} it never has to return anything, just uses error in the panic! arg
    This is a very powerful concept and allows us to create default behavior 
    */

    // Here is another simpler example with our own error
    struct NonPositiveError;
    impl std::fmt::Display for NonPositiveError{
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Item is not positive")
        }
    }

    fn positive(x: i32) -> Result<i32, NonPositiveError> {
        if x>0{
            Ok(x)
        }
        else {
            Err(NonPositiveError)
        }
    }

    let x = -12;

    let positive_x = positive(x).unwrap_or_else(
        |error| {
            if x < 0 {
                println!("Error: {}",error);
                println!("Negating it now.");
                -x
            }
            else {
                x
            }
        }
    );
    println!("original x = {}\npositive x = {}",x, positive_x);

    // The ? operator will automatically pass errors out of a specific function should something fail
    use std::io::Read; // imports read_to_string() method
    fn read_file_error() -> Result<String, std::io::Error> {
        // Try to read file, else throw error
        let mut file_not_exist = File::open("doesnt_exist.txt")?;
        // Create a mutable buffer for file contents
        let mut username = String::new();
        // Pass mutable buffer to read_to_string() method, else throw error
        file_not_exist.read_to_string(&mut username)?;
        Ok(username) // If all succeeds return an OK result
    }

    let result = read_file_error();
    // We can then match on the error to handle anything coming out of std::io::Error
    //Since we reuse the result, we match with the ref keyword.
    // This is because Error and String don't have Copy implemented, so passing them
    // on a match statement will actually move them and make a Result's values disappear
    // This makes sense. A Result is typically not used more than once. But in case you need to reuse its contents,
    // use this ref convention
    match result {
        Ok(ref file_contents)=>println!("{}",file_contents),
        Err(ref error)=>println!("Error: {:?}",error)
    }

    let mut file_buffer = String::new();

    file_buffer = result.unwrap_or_else(
        |error| {
            if error.kind() == ErrorKind::NotFound {
                println!("Creating file (not really)");
                String::from("")
            }
            else {
                println!("Unknown error {}", error);
                String::from("")
            }
        }
    );

    // We can use ? on Options as well
    let text = "The last char\nof the last line\nshould be r\nand nothing else";
    println!("{text}");
    // text.lines() is an iterator, .next() returns an Option. If it fails, return None. Else,
    // call .chars() to create an iterator of characters and .last() gets the last char.
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    println!("Last char of first line: {}",last_char_of_first_line(text).unwrap_or('?'));
    let empty_str = "";
    println!("This one does not work because we pass a null str: {}", last_char_of_first_line(empty_str).unwrap_or('?'));

    // The important thing to remember about ? is that whenever we use it, its parent function must return the
    // same type that the method we use it on does.
    // In the above .next() returns Option<char>
    // We can even alter the main() function like so:
    /*
    fn main() -> Result<(), Box<dyn std::error::Error> > {
        //...
        Ok(())
    }
     */
    // In the above example, we use an error "type" of std::error::Error
    // This is actually a trait. By capturing it in a Box, we allow for all objects that satisfy this trair to
    // be used as a generic. This is known as "Trait Objects" and allows us to define dynamic generics.
    // There is a runtime cost to this method

    // The .expect() method is also very handy when we use parse to define an enum.
    // Below we use .parse() on a &str to return an Enum:
    // pub enum IpAddr {
    //      V4( /* … */ ),
    //      V6( /* … */ ),
    // }
    // .parse() on an explicit typed variable's string implementation for type IpAddr has logic that will load it into
    // the right V4 or V6 variant, and throw an error (like AddrParseError) in a Result.
    // expect() will then panic if it failed.
    use std::net::IpAddr;
    
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded address should be valid.");


}
