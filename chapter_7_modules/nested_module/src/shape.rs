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
            Shape::CircleShape(circ) => circ.area(),
        }
    }

    pub fn get_params(&self) -> std::collections::HashMap<&str, f32>{
        match self {
            Shape::RectangleShape(rect) => rect.get_params(),
            Shape::CircleShape(circ) => circ.get_params()
        }
    }
}
