#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }
    // We can use multiple impl Rectangles or we can put them all in the same block, doesn't matter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // This is an example of an associated function. It's not a function of an instance of the struct,
    // But instead it can be used to return an instance, e.g. String::from()
    // Here Rectangle::square can return a Rectangle, but we will use the Self keyword. Self will be a stand-in for the
    // object type in the current scope
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}


fn main() {
    // Example of struct with method
    let rect1 = Rectangle{
        width: 5,
        height: 10
    };
    //To use the struct's method:
    println!("The area of the rectangle is {}", rect1.area());
    // To print the contents of the struct if you used the derive debug decorator
    println!("The rectangle is: {:?}", rect1);

    let rect2 = Rectangle{
        width: 4,
        height: 4
    };

    //We can use both self and other Rectangles to define methods, e.g.
    println!("Can rect1 hold rect2: {}",rect1.can_hold(&rect2));

    //Instantiate a square:
    let square = Rectangle::square(4);
    println!("The are of square is {}", square.area());
}
