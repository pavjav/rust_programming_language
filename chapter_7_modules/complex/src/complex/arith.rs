/* All the std::ops implementations for addition, subtraction, multiplication, and negation
For Multiplication we show how to pass the rhs and self by reference.
This is cumbersome because we need to define lifetimes for self 'a, and rhs 'b
The compiler needs to know this.
By passing by ref, we increase the performance of multiplication.

*/



/* This is how we would implement multiplication by reference. This can save time by avoiding copying Complex<T>*/
impl<'a,'b, T> std::ops::Mul<&'b super::Complex<T>> for &'a super::Complex<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Copy
{
    type Output = super::Complex<T>;
    fn mul(self, rhs: &'b super::Complex<T>) -> Self::Output {
        super::Complex {
            real: self.real*rhs.real - self.imag*rhs.imag,
            imag: self.real*rhs.imag + self.imag*rhs.real
        }
    }
}
// This implementation is by value, no lifetimes required.
// But we do need to implement Copy for Complex<T>
impl<T> std::ops::Mul for super::Complex<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Copy
{
    type Output = super::Complex<T>;
    fn mul(self, rhs: super::Complex<T>) -> Self::Output {
        super::Complex {
            real: self.real*rhs.real - self.imag*rhs.imag,
            imag: self.real*rhs.imag + self.imag*rhs.real
        }
    }
}

//This is how we multiply a value (self) by reference (rhs)
impl<'b, T> std::ops::Mul<&'b super::Complex<T>> for super::Complex<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Copy
{
    type Output = super::Complex<T>;
    fn mul(self, rhs: &'b super::Complex<T>) -> Self::Output {
        super::Complex {
            real: self.real*rhs.real - self.imag*rhs.imag,
            imag: self.real*rhs.imag + self.imag*rhs.real
        }
    }
}

//This is how we multiply a reference (self) by value (rhs)
impl<'a,T> std::ops::Mul<super::Complex<T>> for &'a super::Complex<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Copy
{
    type Output = super::Complex<T>;
    fn mul(self, rhs: super::Complex<T>) -> Self::Output {
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

// Conjugate for value
impl<T> Conjugate for super::Complex<T>
where 
    T: std::ops::Neg<Output = T> + Copy
{
    type Output = super::Complex<T>;
    fn conj(self) -> Self::Output {
        super::Complex {real: self.real, imag: -self.imag}
    }
}

// Conjugate by reference
impl<T> Conjugate for &super::Complex<T>
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
Here U will take either Complex<T> or &Complex<T>
By setting U=Self, it uses Self by default. This is why we dont use Mul<Complex<T>>
*/
pub trait InnerProduct<U=Self> {
    type Output;
    fn dot(self, rhs: U) -> Self::Output;
}

//No need to param InnerProduct, this is the dot product implementation on Self (so it must copy)
impl<T> InnerProduct for super::Complex<T>
where
    super::Complex<T>: 
        std::ops::Mul<Output = super::Complex<T>> +
        Conjugate<Output = super::Complex<T>>,
{
    type Output = super::Complex<T>;
    fn dot(self, rhs: Self) -> Self::Output {
        self.conj()*rhs
    }
}

//For this type of inner product, we specify that U is a reference to Complex<T> ('b, rhs) and implement it for 
//a reference to Complex<T> (a',self)
impl<'a,'b, T> InnerProduct<&'b super::Complex<T>> for &'a super::Complex<T>
where
    super::Complex<T>: 
        Conjugate<Output = super::Complex<T>>
        + std::ops::Mul<&'b super::Complex<T>, Output = super::Complex<T>>,
    &'a super::Complex<T>:
        Conjugate<Output = super::Complex<T>>
        + 'a
        + 'b,
{
    type Output = super::Complex<T>;
    fn dot(self, rhs: &'b super::Complex<T>) -> Self::Output {
        self.conj()*(rhs)
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
        Copy
        + Conjugate<Output = super::Complex<T>>
        + std::ops::Mul<Output = super::Complex<T>>
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