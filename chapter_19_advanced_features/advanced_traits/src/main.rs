// When we create traits we can also attach associated types to parameterize things like inputs/outputs
// Example

pub trait MyTrait<T> {
    type Input;
    fn multiply(&mut self, rhs: Self::Input)-> Option<T>;
}

impl<T> MyTrait<T> for T 
where 
    T: 
        std::ops::Mul<i32, Output=T> +
        Copy
{
    type Input = i32;
    fn multiply(&mut self, rhs: Self::Input)-> Option<T> {
        Some((*self)*rhs)
    }
}

// There are std::ops traits that are useful, such as add
// traid Add<Rhs=Self> {
//      type Output;
//      fn add(self, rhs: Rhs) -> Self:: Output;
// }
// Here Rhs has a default value of Self, but output must be specified


// This is a straightforward example
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl std::ops::Add for Point{
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

// When we need to use a different input type, say for adding millimeters to meters:
#[derive(PartialEq, Eq, Debug)]
struct Millimeters(u32);
#[derive(PartialEq, Eq, Debug)]
struct Meters(u32);
impl std::ops::Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

/// In case we have multiple methods for a single struct, we can pass self to the Trait::method(&self) to 
/// specify which we are trying to call. But the self.method() always takes precedence
/// 
trait Pilot { fn fly(&self); }
trait Wizard { fn fly(&self); }
struct Human;
impl Pilot for Human { fn fly(&self) { println!("Airplane.") } }
impl Wizard for Human { fn fly(&self) { println!("Broom.") } }
impl Human { fn fly(&self) { println!("Not possible.") } }

/// In case we create traits to capture functions that are not methods:
/// we use the syntax <Type as Trait>::function(args)
/// to access the Animal implementation
/// and Type::function(args) to access the struct implementation
trait Animal { fn baby_name() -> String ;}
struct Dog;
// Dog::baby_name()
impl Dog { fn baby_name() -> String { String::from("Spot") }}
// <Dog as Animal>::baby_name()
impl Animal for Dog { fn baby_name() -> String { String::from("Fido") }}


/// Supertraits are used to introduce trait dependencies
pub trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len+2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len+2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}

/// Using the Newtype pattern to implement external traits on external types
/// We wrap a vector of strings in a struct to implement Display
/// It prints out comma separated values of the vector values
struct Wrapper(Vec<String>);
impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let mut z = 3;
    assert_eq!(z.multiply(12).unwrap(),36);

    let x = Millimeters(32);
    let y = Meters(1);
    assert_eq!(x+y,Millimeters(1032));

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("Baby name for Dog as Animal: {}", <Dog as Animal>::baby_name());
    println!("Baby name for Dog: {}", Dog::baby_name());

    let p = Point{x:1,y:23};
    p.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

}
