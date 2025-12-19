pub mod traits;
pub mod fields;
pub mod integer;
pub mod reals;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn instantiate(){
        let a = integer::Int::new(1);
        assert_eq!(a+a,integer::Int::new(2));
        assert_eq!(a*a,integer::Int::new(1));
        let s = String::new();
    }
}