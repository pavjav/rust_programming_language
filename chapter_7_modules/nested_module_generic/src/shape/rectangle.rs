pub struct Rectangle<T >
where
    T: std::fmt::Display + std::ops::Mul<Output = T> + Copy
{
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T>
where
    T: std::fmt::Display + std::ops::Mul<Output = T> + Copy
{
    fn compute_area(&self) -> T {
        self.width * self.height
    }
}

impl<T> Rectangle<T>
where
    T: std::fmt::Display + std::ops::Mul<Output = T> + Copy
{
    pub fn area(&self) -> T {
        self.compute_area()
    }

    pub fn get_params(&self) -> std::collections::HashMap<&str, &T>{
        let mut params = std::collections::HashMap::new();
        params.insert("width", &self.width);
        params.insert("height", &self.height);
        params
    }
}

impl<T> super::Summary for Rectangle<T> 
where
    T: std::fmt::Display + std::ops::Mul<Output = T> + Copy
{
    fn summarize(&self) -> String {
        format!("Rectangle has width: {} and height: {}", self.width, self.height)
    }
}
