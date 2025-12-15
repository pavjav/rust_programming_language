
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

    let center: (f32,f32) = (0.0,0.0);
    let radius: f32 = 2.0; //or 2.0f32
    let circ = shape::Shape::CircleShape(shape::circle::Circle::new(&center, &radius));

    println!("circle with radius {} has area {}", circ.get_params()["radius"], circ.area());

    let rect= shape::Shape::RectangleShape(
        shape::rectangle::Rectangle{
            width: 2.0f32,
            height: 3.0f32
        }
    );

    println!("{}", rect.summarize());

    println!("rectangle with width {} and heigh {} has area {}", rect.get_params()["width"], rect.get_params()["height"], rect.area());
    println!("{}",circ.summarize());

    let center: (f64,f64) = (2.0,0.0);
    let radius: f64 = 3.0; //or 2.0f32
    let circ64 = shape::Shape::CircleShape(shape::circle::Circle::new(&center, &radius));

    println!("circle with radius {} has area {}", circ64.get_params()["radius"], circ64.area());

    let rect64= shape::Shape::RectangleShape(
        shape::rectangle::Rectangle{
            width: 5.0f64,
            height: 3.7f64
        }
    );



    println!("{}", rect64.summarize());

    println!("rectangle with width {} and heigh {} has area {}", rect64.get_params()["width"], rect64.get_params()["height"], rect64.area());
    println!("{}",circ64.summarize());
}
