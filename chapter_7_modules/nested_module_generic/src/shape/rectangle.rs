pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    fn compute_area(&self) -> f32 {
        self.width * self.height
    }

    pub fn area(&self) -> f32 {
        self.compute_area()
    }

    pub fn get_params(&self) -> std::collections::HashMap<&str, f32>{
        let mut params = std::collections::HashMap::new();
        params.insert("width", self.width);
        params.insert("height", self.height);
        params
    }
}

impl super::Summary for Rectangle {
    fn summarize(&self) -> String {
        format!("Rectangle has width: {} and height: {}", self.get_params()["width"], self.get_params()["height"])
    }
}
