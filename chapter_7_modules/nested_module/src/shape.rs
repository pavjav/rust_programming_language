
pub mod rectangle;
pub mod circle;

pub enum Shape {
    RectangleShape(rectangle::Rectangle),
    CircleShape(circle::Circle)
}

impl Shape {
    pub fn area(&self) -> f32 {
        match self {
            Shape::RectangleShape(rect) => rect.area(),
            Shape::CircleShape(circ) => circ.area()
        }
    }

    pub fn get_params(&self) -> std::collections::HashMap<&str, f32>{
        match self {
            Shape::RectangleShape(rect) => rect.get_params(),
            Shape::CircleShape(circ) => circ.get_params()
        }
    }

    pub fn summarize(&self) -> String {
        match self {
            Shape::CircleShape(rect) => rect.summarize(),
            Shape::RectangleShape(circ) => circ.summarize()
        }
    }
}

// This is an example of a virtual function. There is a default behavior if the variant does not define it.
// We still have to add a impl super::Summary for Circle {}, for e.g. to use this default behavior.
// And we still need to implement the method to the Enum as above.
pub trait Summary{
    fn summarize(&self) -> String {
        String::from("No summary available")
    }
}

