struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct Point (i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@email.com"),
        sign_in_count: 1,
    };

    //We can modify the fields of user1 because it is mutable
    user1.email = String::from("someone_else@email.com");
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);

    //Creating a new instance of User using struct update syntax. This copies the remaining fields from user1.
    let user2 = User {
        email: String::from("another@email.com"),
        ..user1
    };

    println!("User2 Username: {}", user2.username);
    println!("User2 Email: {}", user2.email);

    // Creating a User using a function
    let user3 = create_user(String::from("someone_else@email.com"), String::from("someusername456"));
    println!("User3 Username: {}", user3.username);
    println!("User3 Email: {}", user3.email);

    // Creating a User using field init shorthand syntax
    let user4 = create_user_field_init(String::from("someone@email.com"), String::from("someusername789"));
    println!("User4 Username: {}", user4.username);
    println!("User4 Email: {}", user4.email);

    // Using a tuple struct
    let black = Color(0,0,0,);
    let origin = Point(0,0,0);
    println!("Color - red: {}, green: {}, blue: {}", black.0, black.1, black.2);
    println!("Point - x: {}, y: {}, z: {}", origin.0, origin.1, origin.2);
    // Tuple structs behave like regular tuples, but have named types. This assignment works because the fields are unnamed.
    let Point(x, y, z) = origin;
    println!("Destructured Point - x: {}, y: {}, z: {}", x, y, z);

    // Using a unit-like struct
    let subject = AlwaysEqual;
    println!("Created an instance of AlwaysEqual struct.");

}

// Long form
fn create_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Field init shorthand syntax
fn create_user_field_init(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

