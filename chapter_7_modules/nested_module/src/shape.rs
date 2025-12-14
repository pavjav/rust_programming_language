pub mod rectangle;
pub mod circle;

enum Shapes {
    Rectangle(rectangle::Rectangle),
    Circle(circle::Circle)
}
