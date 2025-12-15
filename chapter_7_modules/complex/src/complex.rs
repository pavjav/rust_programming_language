mod arith;
/*
Create the public struct called Complex.
Fields:
    real: T
        The real part of the complex number
    imag: T 
        The imaginary part of the complex number
*/
#[derive(Copy, Clone, Debug)]
pub struct Complex <T> {
    real: T,
    imag: T
}


impl<T> Complex<T> 
where
    T: 
        Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Neg<Output = T>,
        
    Complex<T>:
        Copy
        + std::ops::Add<Output = Complex<T>>
        + std::ops::Sub<Output = Complex<T>>
        + std::ops::Mul<Output = Complex<T>>
        + std::ops::Div<Output = Complex<T>>
{
    pub fn new(real: T, imag: T) -> Self {
        Complex { real: real, imag: imag }
    }

    pub fn inverse(self) -> Self{
        arith::Inverse::inverse(self)
    }

    pub fn conj(self) -> Self{
        arith::Conjugate::conj(self)
    }

    pub fn dot(self, rhs: Self) -> Self{
        arith::InnerProduct::dot(self, rhs)
    }

    pub fn modulus(self) -> T {
        arith::Modulus::modulus(self)
    }
    
}