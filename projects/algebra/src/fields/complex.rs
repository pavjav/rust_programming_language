#[derive(Copy, Clone, Debug)]
pub struct Complex <T>
where
{
    real: T,
    imag: T
}

impl<T> crate::traits::FieldSet<Self> for Complex<T>
where
    T:
        Copy +
        std::ops::Add<T,Output=T> +
        std::ops::Sub<T,Output=T> +
        std::ops::Mul<T,Output=T>
{}

impl<T> crate::traits::GroupSet for Complex<T>
where
    T:
        Copy +
        std::ops::Add<T,Output=T> +
        std::ops::Sub<T,Output=T> +
        std::ops::Mul<T,Output=T>
{}
impl<T> crate::traits::SemiGroupSet for Complex<T>
where
    T:
        Copy +
        std::ops::Add<T,Output=T> +
        std::ops::Sub<T,Output=T> +
        std::ops::Mul<T,Output=T>
{}
impl<T> crate::traits::AbelianSemiGroupSet for Complex<T>
where
    T:
        Copy +
        std::ops::Add<T,Output=T> +
        std::ops::Sub<T,Output=T> +
        std::ops::Mul<T,Output=T>
{}
impl<T> crate::traits::AbelianGroupSet for Complex<T>
where
    T:
        Copy +
        std::ops::Add<T,Output=T> +
        std::ops::Sub<T,Output=T> +
        std::ops::Mul<T,Output=T>
{}
impl<T> crate::traits::HasAddIdentitySet for Complex<T>{
    fn id_add(self)->Self {
        id_add()
    }
}
impl<T> crate::traits::HasAddInverseSet for Complex<T>{
    fn negative(self)->Self {
        -self
    }
}
impl<T> crate::traits::HasMulIdentitySet for Complex<T>{
    fn id_mul(self)->Self {
        id_mul()
    }
}
impl<T> crate::traits::HasMulInverseSet for Complex<T>{
    fn inverse(self)->Self {
        
    }
}

pub fn new<T>(x:T,y:T) -> Complex<T>
where
    T:
        std::ops::Mul<T,Output=T> +
        std::ops::Add<T,Output=T>
{
    Complex{real: x, imag: y}
}

pub fn id_add<T>()->Complex<T> {
    Complex{real:0,imag:0}
}

pub fn id_mul<T>()->Complex<T> {
    Complex{real:1,imag:0}
}

impl<T> std::ops::Add for Complex<T>
where
    T: std::ops::Add<Output = T> + Copy
{
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + self.imag
        }
    }
}


impl<T> std::ops::Mul<Complex<T>> for Complex<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Copy
{
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Self::Output {
        Complex {
            real: self.real*rhs.real - self.imag*rhs.imag,
            imag: self.real*rhs.imag + self.imag*rhs.real
        }
    }
}

impl<T> std::ops::Sub for Complex<T>
where
    T: std::ops::Sub<Output = T> + Copy
{
    type Output = Complex<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - self.imag
        }
    }
}

impl<T> std::ops::Neg for Complex<T>
where
    T: std::ops::Neg<Output = T> + Copy
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag
        }
    }
}

impl<T> Inverse for Complex<T>
where
    Complex<T>: Modulus<T>,
    T: 
        Copy +
        std::ops::Div<Output = T> +
        std::ops::Neg<Output = T>
{
    type Output = Complex<T>;
    fn inverse(self)->Self::Output {
        Complex{
            real: self.real/(self.modulus()),
            imag: -self.imag/(self.modulus())
        }
    }
}