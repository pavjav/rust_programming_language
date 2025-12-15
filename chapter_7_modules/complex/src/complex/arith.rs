/* All the std::ops implementations for addition, subtraction, multiplication, and negation*/
impl<T> std::ops::Mul for super::Complex<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Copy
{
    type Output = super::Complex<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        super::Complex {
            real: self.real*rhs.real - self.imag*rhs.imag,
            imag: self.real*rhs.imag + self.imag*rhs.real
        }
    }
}

impl<T> std::ops::Add for super::Complex<T>
where
    T: std::ops::Add<Output = T> + Copy
{
    type Output = super::Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        super::Complex {
            real: self.real + rhs.real,
            imag: self.imag + self.imag
        }
    }
}

impl<T> std::ops::Sub for super::Complex<T>
where
    T: std::ops::Sub<Output = T> + Copy
{
    type Output = super::Complex<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        super::Complex {
            real: self.real - rhs.real,
            imag: self.imag - self.imag
        }
    }
}

impl<T> std::ops::Neg for super::Complex<T>
where
    T: std::ops::Neg<Output = T> + Copy
{
    type Output = super::Complex<T>;
    fn neg(self) -> Self::Output {
        super::Complex {
            real: -self.real,
            imag: -self.imag
        }
    }
}

/*
Conjugate trait + implementation. Takes z to z* (complex conjugate).
*/
pub trait Conjugate {
    type Output;
    fn conj(self)->Self::Output;
}

impl<T> Conjugate for super::Complex<T>
where 
    T: std::ops::Neg<Output = T> + Copy
{
    type Output = super::Complex<T>;
    fn conj(self) -> Self::Output {
        super::Complex {real: self.real, imag: -self.imag}
    }
}

/*
Inner Product trait. takes (z,w) to z*w
*/
pub trait InnerProduct {
    type Output;
    fn dot(self, rhs: Self) -> Self::Output;
}

impl<T> InnerProduct for super::Complex<T>
where
    super::Complex<T>: 
        std::ops::Mul<Output = super::Complex<T>> +
        Conjugate<Output = super::Complex<T>>
{
    type Output = super::Complex<T>;
    fn dot(self, rhs: Self) -> Self::Output {
        self.conj()*rhs
    }
}

/*
Modulus trait, takes z to z*z = (zz*)* = (dot(z,z))* (T value)
*/
pub trait Modulus<T> {
    fn modulus(self) -> T;
}

impl<T> Modulus<T> for super::Complex<T> 
where 
    super::Complex<T>:
        Copy +
        Conjugate<Output = super::Complex<T>> +
        std::ops::Mul<Output = super::Complex<T>>
{
    fn modulus(self) -> T {
        (self.dot(self)).conj().real
    }
}

/*
Inverse trait, takes z to 1/z
*/
pub trait Inverse {
    type Output;
    fn inverse(self)-> Self::Output;
}
impl<T> Inverse for super::Complex<T>
where
    super::Complex<T>: Modulus<T>,
    T: 
        Copy +
        std::ops::Div<Output = T> +
        std::ops::Neg<Output = T>
{
    type Output = super::Complex<T>;
    fn inverse(self)->Self::Output {
        super::Complex{
            real: self.real/(self.modulus()),
            imag: -self.imag/(self.modulus())
        }
    }
}

/*
Div trait, takes (z,w) to z/w
*/
impl<T> std::ops::Div for super::Complex<T>
where
    super::Complex<T>:
        Copy +
        Inverse<Output = super::Complex<T>> +
        std::ops::Mul<Output = super::Complex<T>>
{
    type Output = super::Complex<T>;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl<T> std::cmp::PartialEq for super::Complex<T> 
where
    T: std::cmp::PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        (self.real == other.real) && (self.imag == other.imag)
    }
}