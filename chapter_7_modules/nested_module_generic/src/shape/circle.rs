mod point;

pub struct Circle<T>
where
    T: Copy
{
    center: point::Point<T>,
    radius: T,
}



/*
impl<f32> super::ComputeArea for Circle<f32>{
    fn compute_area(&self) -> f32 {
        std::f32::consts::PI * self.radius
    }
}

impl<f64> super::ComputeArea for Circle<f64>{
    fn compute_area(&self) -> f32 {
        std::f64::consts::PI * self.radius
    }
}

impl<i64> super::ComputeArea for Circle<i64>{
    fn compute_area(&self) -> i64 {
        3i64 * self.radius
    }
}
*/

pub trait DynamicAreaCompute {
    fn dynamic_area_compute(&self)->Self;
}

impl DynamicAreaCompute for f32 {
    fn dynamic_area_compute(&self) -> Self {
        std::f32::consts::PI * self.powf(2.0f32)
    }
}

impl DynamicAreaCompute for f64 {
    fn dynamic_area_compute(&self) -> Self {
        std::f64::consts::PI * self.powf(2.0f64)
    }
}

impl<T> Circle<T> 
where
    T: Copy + std::ops::Mul<Output = T> + DynamicAreaCompute
{

    pub fn area(&self) -> T {
        //self.compute_area()
        //self.compute_area()
        DynamicAreaCompute::dynamic_area_compute(&self.radius)
    }


    //pub fn new(x: &T, y: &T, radius: &T) -> Circle<T> {
    //    Circle { center: center::point::Point::new(x,y), radius: radius}
    //}
    pub fn new( coord: &(T,T), radius: &T) -> Circle<T> {
        Circle { center: point::Point::new(&coord.0, &coord.1), radius: *radius}
    }

    pub fn get_params(&self) -> std::collections::HashMap<&str, &T>{
        let mut params = std::collections::HashMap::new();
        params.insert("center_x", &self.center.x);
        params.insert("center_y", &self.center.y);
        params.insert("radius", &self.radius);
        params
    }
}

impl<T> super::Summary for Circle<T> 
where
    T: Copy + std::fmt::Display 
{}