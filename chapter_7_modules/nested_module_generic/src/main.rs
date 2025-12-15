use crate::shape::Summary;

mod shape;

fn main() {

    /*
    Module structure:
    ├── src
│   ├── main.rs
│   ├── shape
│   │   ├── circle
│   │   │   └── point
│   │   │       └── mod.rs
│   │   ├── circle.rs
│   │   └── rectangle.rs
│   └── shape.rs

    
    */

    let circ = shape::Shape::CircleShape(shape::circle::Circle::new(&(0.0,0.0), &2.0));

    println!("circle with radius {} has area {}", circ.get_params()["radius"], circ.area());

    let rect= shape::Shape::RectangleShape(
        shape::rectangle::Rectangle{
            width: 2.0,
            height: 3.0
        }
    );

    println!("rectangle with width {} and heigh {} has area {}", rect.get_params()["width"], rect.get_params()["height"], rect.area());
    println!("{}",circ.summarize());
}
