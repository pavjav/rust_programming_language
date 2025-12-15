pub struct Point <T>
where T: Copy
{
    pub x: T,
    pub y: T,
    _z: Option <T>
}

impl<T> Point<T> 
where
    T: Copy
{
    pub fn new(x: &T, y: &T) -> Point<T> {
        Point{
            x: *x,
            y: *y,
            _z: None
        }
    }
}