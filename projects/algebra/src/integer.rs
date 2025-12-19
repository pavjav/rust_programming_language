#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Int {
    elem: isize
}

impl crate::traits::AbelianGroupSet for Int{}
impl crate::traits::AbelianMonoidSet for Int{}
impl crate::traits::AbelianSemiGroupSet for Int{}

impl crate::traits::CommutativeMonoidSet for Int{}
impl crate::traits::MonoidSet for Int{}
impl crate::traits::SemiGroupSet for Int{}

impl crate::traits::RngSet for Int{}
impl crate::traits::RingSet for Int{}
impl crate::traits::CommutativeRingSet for Int{}

impl crate::traits::LeftModuleSet<Int> for Int{}
impl crate::traits::RightModuleSet<Int> for Int{}
impl crate::traits::ModuleSet<Int> for Int{}

impl Int {
    pub fn new(elem: isize) -> Int {
        Int{elem: elem}
    }
}


impl std::ops::Mul<Int> for Int {
    type Output = Int;
    fn mul(self, rhs: Int) -> Self::Output {
        Int{elem: self.elem * rhs.elem}
    }
}

impl std::ops::Add<Int> for Int {
    type Output = Int;
    fn add(self, rhs: Int) -> Self::Output {
        Int{elem: self.elem + rhs.elem}
    }
}

impl std::ops::Neg for Int {
    type Output = Int;
    fn neg(self) -> Self::Output {
        Int{elem: -self.elem}
    }
}

impl std::ops::Sub<Int> for Int {
    type Output = Int;
    fn sub(self, rhs: Int) -> Self::Output {
        Int{elem: self.elem - rhs.elem }
    }
}

impl crate::traits::HasAddIdentitySet for Int {
    fn id_add(self)->Self {
        Int{elem: 0isize}
    }
}

impl crate::traits::HasMulIdentitySet for Int {
    fn id_mul(self)->Self {
        Int{elem: 1isize}
    }
}

