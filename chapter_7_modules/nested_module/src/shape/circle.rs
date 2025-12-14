mod point;

pub struct Circle{
    center: point::Point,
    radius: f32,
}

impl Circle {
    fn compute_area(&self) -> f32 {
        std::f32::consts::PI * self.radius
    }

    pub fn area(&self) -> f32 {
        self.compute_area()
    }

    pub fn new(center: &(f32,f32), radius: &f32) -> Circle {
        Circle { center: point::Point::new(&center.0, &center.1), radius: *radius }
    }

    pub fn get_params(&self) -> std::collections::HashMap<&str, f32>{
        let mut params = std::collections::HashMap::new();
        params.insert("center_x", self.center.x);
        params.insert("center_y", self.center.y);
        params.insert("radius", self.radius);
        params
    }
}