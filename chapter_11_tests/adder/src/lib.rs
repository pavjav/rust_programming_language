pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value <1 || value> 100 {
            panic!(
                "Guess must be between 1 and 100, got {}",
                value
            );
        }
        println!("Value is valid: {}",value); //This won't print during a test, we must add the --show-output arg
        Guess {value:value}
    }

    pub fn new2(value: i32) -> Guess {
        if value <1 {
            panic!(
                "Guess must greater than or equal to 1, got {}",
                value
            );
        }
        else if value > 100 {
            panic!("Guess must be less than or equal to 100, got {}", value);
        }
        Guess {value:value}
    }

}


// The #[cfg(tests)] attribute allows us to only compile this module on test. Otherwise when we run cargo build or publish
// These tests wont take up space in the binary
// This attribute also gives tests access to private functions, and structs
// Above we use both pub and private functions/structs, and the tests module can access them.
#[cfg(test)]
mod tests {
    use super::*;
    // assert_eq! is a macro that just ensures two values are the same.
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    //assert_ne! works by asserting two values are different
    #[test]
    fn exploration2() {
        let result = 2 + 2;
        assert_ne!(result, 5);
    }

    //We can call panic! to return errors
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // We can use assert! to return an Error on a boolean expression
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };
        
        assert!(!smaller.can_hold(&larger));
    }

    fn greeting(name: &str) -> String {
        String::from("Hello")
    }
    // To add a custom failure message
    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain naem, value was {result}"
        );
    }

    // To make a test pass on error we use the #[should_panic] attribute
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // With the error message thrown by panic! we can use a substring
    // expected = "must be less than or equal to 100"
    // Since this is a substring of the error message we can have a panic! test that passes depending on the message
    // 

    #[test]
    #[should_panic(expected = "must be less than or equal to 100")]
    fn greater_than_100_specific_error() {
        Guess::new2(200);
    }

    // Using Result<T,E> in Tests

    #[test]
    fn it_works_result() -> Result<(),String> {
        if 2 + 2 == 4{
            Ok(())
        }
        else {
            Err(String::from("two plus two does not equal 4"))
        }
    }

    // To capture the stdout of a successful Guess::new call, we use the --show-output arg:
    // cargo test -- --show-output
    #[test]
    fn test_stdout() {
        Guess::new(50);
    }

    // When we use the #[ignore] attribute the test wont run unless specifically requested
    // We do this for expensive tests
    // We can specifically run ignored tests with cargo test -- --ignored

    #[test]
    #[ignore]
    fn expensive_test() {

    }

}
