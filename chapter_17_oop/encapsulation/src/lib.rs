pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }
    
    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    fn new(v: Vec<i32>) -> AveragedCollection {
        let mut result = AveragedCollection {
            list: v,
            average: 0.0
        };
        result.update_average();
        result

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut z = AveragedCollection::new(vec![1,2,3]);
        println!("Average: {}", z.average());
        z.add(4);
        println!("Average: {}", z.average());
        z.remove();
        println!("Average: {}", z.average());
    }
}
