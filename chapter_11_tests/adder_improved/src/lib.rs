pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn add_private(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Example<T> {
    pub value: T
}

impl<T> Example<T>
{
    pub fn get_ref(&self)-> &T {
        &self.value
    }
}

impl<T> Example<T>
where
    T: Copy
{
    pub fn get_value(&self)-> T {
        self.value
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn public(){
        assert_eq!(4, add(2,2,)); // Can access public function  
    }
    #[test]
    fn private_add() {
        assert_eq!(4, add_private(2,2)); // Can also access private functions
    }
}