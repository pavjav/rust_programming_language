extern crate nalgebra as na;

impl<R,C> crate::traits::AbelianGroupSet for na::Matrix<super::Real,R,C, ArrayStorage>{}
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