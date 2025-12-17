/*
Here we use a generic T to define this function.
It takes list of type &[T]. This is an array of T values by reference.

The process of monomorphization turns generic code into specific code. The compiler will do this for us
if we follow the proper syntax. We use traits in the where clause to ensure that we can perform operations/call methods 
in the bodies of our generic objects.
*/
fn largest<T>(list: &[T]) -> &T 
where 
    T: std::cmp::PartialOrd
{
    let mut largest = &list[0];
    // largest is a new variable of type &T that points to list[0]

    for item in list {
        if item > largest { // If larger, reassign largest to point to item
            largest = item;
        }
    }
    largest
    // This returns a reference to an element in the original array
}

//Single generic
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
// Two generics with default behavior on second
#[derive(Debug)]
struct PointTwo<T,U=T> {
    x: T,
    y: U
}

// Non-generic trait to ensure we can take the square root.
// If we do not implement something, we can use a default behavior
// We can also require Self and/or generics to be able to satisfy other traits.
// Below we require Self to be Sized so that the default behavior can return self.
trait SquareRoot
where
    Self: Sized,
{
    fn my_sqrt(self: Self) -> Self{
        self
    }
}

// Implement for f64 using its native method
impl SquareRoot for f64 {
    fn my_sqrt(self: f64) -> f64 {
        self.sqrt()
    }
}

impl SquareRoot for i32{}

// Implementation on original struct
impl<T> Point<T> 
where
    T: 
    std::ops::Mul<Output = T>
    + std::ops::Add<Output = T>
    + SquareRoot
    + Copy
{
    // Take self by reference, return a reference to attribute x
    fn x(&self)-> &T {
        &self.x
    }

    fn distance_from_origin(&self) -> T {
        ((self.x*self.x) + (self.y*self.y)).my_sqrt()
    }
}

// We can even return types that implement traits

struct Example {
    pub value: String,
}

trait Summary {
    fn summarize(self) -> String;
}


fn main() {
    let array = vec![34,50,25,100];
    println!("Largest element of {:?} is: {}",array,largest(&array));

    let float = Point {x: 1.0, y: 4.0};
    println!("{:?}",float);
    println!("Distance from origin is: {}",float.distance_from_origin());

    let int = Point {x: 1, y: 4};
    println!("{:?}",int);
    println!("Distance from origin is: {}",int.distance_from_origin());
    //For integers this is obviously wrong, above returns 1^2 + 4^2 = 17.
    //This is because my_sqrt() just returns self

    let mixed = PointTwo{x:1.0, y: "blah"};
    let same: PointTwo<f64> = PointTwo{x:1.0,y:1.0};
    println!("{:?}", mixed);
    println!("{:?}", same)



}
