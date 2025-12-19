pub mod matrix;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Real {
    elem: f64
}

impl crate::traits::AbelianGroupSet for Real{}
impl crate::traits::AbelianMonoidSet for Real{}
impl crate::traits::AbelianSemiGroupSet for Real{}

impl crate::traits::GroupSet for Real{}
impl crate::traits::CommutativeGroupSet for Real{}
impl crate::traits::MonoidSet for Real{}
impl crate::traits::CommutativeMonoidSet for Real{}
impl crate::traits::SemiGroupSet for Real{}
impl crate::traits::CommutativeSemiGroupSet for Real{}


impl crate::traits::RngSet for Real{}
impl crate::traits::RingSet for Real{}
impl crate::traits::CommutativeRingSet for Real{}
impl crate::traits::DivisionRingSet for Real{}
impl crate::traits::FieldSet for Real{}

impl crate::traits::LeftModuleSet<Real> for Real{}
impl crate::traits::RightModuleSet<Real> for Real{}
impl crate::traits::ModuleSet<Real> for Real{}

impl Real {
    pub fn new(elem: f64) -> Real {
        Real{elem: elem}
    }
}


impl std::ops::Mul<Real> for Real {
    type Output = Real;
    fn mul(self, rhs: Real) -> Self::Output {
        Real{elem: self.elem * rhs.elem}
    }
}

impl std::ops::Add<Real> for Real {
    type Output = Real;
    fn add(self, rhs: Real) -> Self::Output {
        Real{elem: self.elem + rhs.elem}
    }
}

impl std::ops::Neg for Real {
    type Output = Real;
    fn neg(self) -> Self::Output {
        Real{elem: -self.elem}
    }
}

impl std::ops::Sub<Real> for Real {
    type Output = Real;
    fn sub(self, rhs: Real) -> Self::Output {
        Real{elem: self.elem - rhs.elem }
    }
}

impl std::ops::Div<Real> for Real {
    type Output = Real;
    fn div(self, rhs: Real) -> Self::Output {
        Real{elem: self.elem/rhs.elem }
    }
}

impl crate::traits::HasAddIdentitySet for Real {
    fn id_add(self)->Self {
        Real{elem: 0.0f64}
    }
}

impl crate::traits::HasMulIdentitySet for Real {
    fn id_mul(self)->Self {
        Real{elem: 1.0f64}
    }
}

impl crate::traits::HasMulInverseSet for Real {
    fn inverse(self)->Self {
        Real{elem: 1.0f64/self.elem}
    }
}

