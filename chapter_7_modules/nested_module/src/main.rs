
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

    There are two ways to create a module. The first is to use module_name.rs. If you want to nest, you add more 
    modules under module_name/ directory. The second way is to use module_name/mod.rs,

    To import a module, you simply write mod module_name;
    Your import can be public if you want it accessible. Typically submodules are inaccessible and the parent module holds
    everything you need.

    For example shape.rs does a pub mod rectangle; pub mod circle; so those modules are accessible if we only import shape.
    This allows those objects to be accessed directly.

    However, circle does a private import of point. This means we cannot instantiate Point without using some circle method.
    
    shape::Shape is a public Enum with two variants:
    RectangleShape(shape::rectangle::Rectangle)
    CircleShape(shape::circle::Circle)
    It has two public methods: area() and get_params()
    These both match on either Enum variant and call their respective get_params() methods

    shape::rectangle has a public struct shape::rectangle::Rectangle

    This struct has public attributes width and height.

    It has a private compute_area() and public area(), get_params() methods. This way shape::Shape cannot access
    compute_area() but can access the other two.

    shape::circle has a public struct called shape::circle::Circle.
    This struct has two private attributes center (f32) and radius of type Point.

    It imports point privately, so we implement a new(&(f32,f32), &f32) method. This returns an instance of Circle.

    shape::circle::point has a struct called Point with another new() method. Point has two public f32 attributes called x,y.
    and a private Option<f32> attribute called z.
    Because x,y are public we can instantiate a new 2-D point with either:
    Point::new(&(x,y)) or Point{x:x,y:y, _z:None}. The latter is what new essentially does.

    z is never used, its just meant to highlight how we can create optional fields.

    Note on immutability:
    Mutability is inherited by the owner of the instance. This means that you cannot specify which fields are/aren't mutable.
    You must instead design the objects with this in mind. When you need to update your object's attributes you use

    impl Structure {
        fn func(&mut self) {
            // use self.field
        }
    }

    If you really need to make only certain fields mutable, you can use std::cell::Cell

    struct Structure {
        x: i32,
        y: std::cell::Cell<i32>
    }

    impl Structure {
        fn func(&self) {
            self.y.set(self.y.get() + 1);
        }
    }

    But this implements !Sync which means we can no longer multithread with this struct. Be aware of the tradeoff.

    This project also shows how we can use traits to act as virtual functions with a default behavior.
    Check shape.rs for the details.    

    
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
