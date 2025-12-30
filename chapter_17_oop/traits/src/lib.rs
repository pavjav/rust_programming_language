pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Let's create a trait called Draw that requires a draw() method
pub trait Draw {
    fn draw(&self);
}

// Now this struct contains components that is a vector of Box pointers of objects that implement Draw
pub struct Screen {
    pub components: Vec<Box <dyn Draw>>
}

// Screen will implement a run() method that iterates over the components and calls draw on each
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}
impl Draw for Button {
    fn draw(&self) {

    }
}

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {

    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![String::from("Yes"), String::from("No")]
                }),
                Box::new(Button {
                    width:50,
                    height: 10,
                    label: String::from("OK")
                })
            ]
        };

        screen.run();
    }
}
