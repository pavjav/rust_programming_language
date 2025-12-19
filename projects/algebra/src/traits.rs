pub trait HasMulIdentitySet<U=Self> {
    fn id_mul(self)->U;
}

pub trait HasAddIdentitySet<U=Self> {
    fn id_add(self)->U;
}

pub trait HasMulInverseSet<U=Self> {
    fn inverse(self)->U;
}
/// (U,*) forms a semigroup (associative multiplication)
/// We do not have a trait for a magma because this is basically just std::ops::Mul
pub trait SemiGroupSet<U=Self>
where
    U: std::ops::Mul<U,Output=U>
{}

/// (U,*) forms a commuative semigroup
pub trait CommutativeSemiGroupSet<U=Self>
where
    U:
        std::ops::Mul<U,Output=U> +
        SemiGroupSet<U>
{}

/// (U,+) forms an abelian semigroup (associative and commutative)
pub trait AbelianSemiGroupSet<U=Self>
where
    U: std::ops::Add<U, Output=U>
{}

/// (U,*) forms a monoid (identity exists)
pub trait MonoidSet<U=Self>
where
    U: 
        SemiGroupSet<U> + 
        HasMulIdentitySet<U> +
        std::ops::Mul<U, Output=U>
{}

/// (U,*) forms a commutative monoid
pub trait CommutativeMonoidSet<U=Self>
where
    U:
        MonoidSet<U> +
        SemiGroupSet<U> +
        HasMulIdentitySet<U> +
        std::ops::Mul<U, Output=U>
{}

/// (U,+) forms a monoid (identity exists)
pub trait AbelianMonoidSet<U=Self>
where
    U: 
        AbelianSemiGroupSet<U> +
        HasAddIdentitySet<U> +
        std::ops::Add<U, Output=U>
{}

/// (U,*) forms a group (identity, inverse, and division)
pub trait GroupSet<U=Self>
where
    U: 
        SemiGroupSet<U> +
        HasMulIdentitySet<U> +
        HasMulInverseSet<U> +
        MonoidSet<U> +
        std::ops::Mul<U,Output=U> +
        std::ops::Div<U,Output=U>
{}

/// (U,*) forms a commutative group (identity, inverse, and division)
pub trait CommutativeGroupSet<U=Self>
where
    U:  
        GroupSet<U> +
        SemiGroupSet<U> +
        HasMulIdentitySet<U> +
        HasMulInverseSet<U> +
        MonoidSet<U> +
        std::ops::Mul<U,Output=U> +
        std::ops::Div<U,Output=U>
{}

/// (U,+) forms a group (identity, subtraction, and negation)
pub trait AbelianGroupSet<U=Self>
where
    U: 
        AbelianSemiGroupSet<U> +
        AbelianMonoidSet<U> +
        HasAddIdentitySet<U> + 
        std::ops::Add<U, Output=U> + 
        std::ops::Sub<U, Output=U> + 
        std::ops::Neg
{}

/// (U,+,*) forms a Rng. (U,+) an abelian group (U,*) a semigroup
pub trait RngSet<U=Self>
where
    U: 
        SemiGroupSet<U> +
        AbelianGroupSet<U> +
        AbelianMonoidSet<U> +
        AbelianSemiGroupSet<U> +
        HasAddIdentitySet<U> +
        std::ops::Mul<U,Output=U> +
        std::ops::Add<U,Output=U> +
        std::ops::Sub<U, Output=U> +
        std::ops::Neg
{}

/// (U,+,*) forms a Ring. (U,+) an abelian group (U,*) a monoid
pub trait RingSet<U=Self>
where
    U:  
        RngSet<U> +
        MonoidSet<U> +
        SemiGroupSet<U> +
        AbelianGroupSet<U> +
        AbelianMonoidSet<U> +
        AbelianSemiGroupSet<U> +
        HasAddIdentitySet<U> +
        HasMulIdentitySet<U> +
        std::ops::Mul<U,Output=U> +
        std::ops::Add<U,Output=U> +
        std::ops::Sub<U, Output=U> +
        std::ops::Neg
{}

pub trait DivisionRingSet<U=Self>
where
    U:  
        RingSet<U> +
        RngSet<U> +
        GroupSet<U> +
        MonoidSet<U> +
        SemiGroupSet<U> +
        AbelianGroupSet<U> +
        AbelianMonoidSet<U> +
        AbelianSemiGroupSet<U> +
        HasAddIdentitySet<U> +
        HasMulIdentitySet<U> +
        HasMulInverseSet<U> +
        std::ops::Mul<U,Output=U> +
        std::ops::Div<U,Output=U> +
        std::ops::Add<U,Output=U> +
        std::ops::Sub<U, Output=U> +
        std::ops::Neg
{}

/// (U,+,*) forms a commutative ring. 
pub trait CommutativeRingSet<U=Self>
where
    U:  
        RingSet<U> +
        RngSet<U> +
        CommutativeMonoidSet<U> +
        MonoidSet<U> +
        SemiGroupSet<U> +
        AbelianGroupSet<U> +
        AbelianMonoidSet<U> +
        AbelianSemiGroupSet<U> +
        HasAddIdentitySet<U> +
        HasMulIdentitySet<U> +
        std::ops::Mul<U,Output=U> +
        std::ops::Add<U,Output=U> +
        std::ops::Sub<U, Output=U> +
        std::ops::Neg
{}

/// (U,+,*) forms a field
pub trait FieldSet<U=Self>
where
    U:  
        RingSet<U> +
        RngSet<U> +
        CommutativeGroupSet<U> +
        GroupSet<U> +
        CommutativeMonoidSet<U> +
        MonoidSet<U> +
        SemiGroupSet<U> +
        AbelianGroupSet<U> +
        AbelianMonoidSet<U> +
        AbelianSemiGroupSet<U> +
        HasAddIdentitySet<U> +
        HasMulIdentitySet<U> +
        HasMulInverseSet<U> +
        std::ops::Mul<U,Output=U> +
        std::ops::Div<U, Output=U> +
        std::ops::Add<U,Output=U> +
        std::ops::Sub<U, Output=U> +
        std::ops::Neg
{}

pub trait LeftModuleSet<R,U=Self>
where
    U:
        AbelianGroupSet<U> +
        AbelianMonoidSet<U> +
        AbelianSemiGroupSet<U> +
        HasAddIdentitySet<U> +
        std::ops::Add<U,Output=U> +
        std::ops::Sub<U, Output=U> +
        std::ops::Neg,
    R:
        std::ops::Mul<U,Output=U> +
        RingSet<R> +
        RngSet<R> +
        MonoidSet<R> +
        SemiGroupSet<R> +
        AbelianGroupSet<R> +
        AbelianMonoidSet<R> +
        AbelianSemiGroupSet<R> +
        HasAddIdentitySet<R> +
        HasMulIdentitySet<R> +
        std::ops::Mul<R,Output=R> +
        std::ops::Add<R,Output=R> +
        std::ops::Sub<R, Output=R> +
        std::ops::Neg
{}

pub trait RightModuleSet<R,U=Self>
where
    U:
        AbelianGroupSet<U> +
        AbelianMonoidSet<U> +
        AbelianSemiGroupSet<U> +
        HasAddIdentitySet<U> +
        std::ops::Mul<U,Output=U> +
        std::ops::Add<U,Output=U> +
        std::ops::Sub<U, Output=U> +
        std::ops::Neg +
        std::ops::Mul<R,Output=U>,
    R:
        RingSet<R> +
        RngSet<R> +
        MonoidSet<R> +
        SemiGroupSet<R> +
        AbelianGroupSet<R> +
        AbelianMonoidSet<R> +
        AbelianSemiGroupSet<R> +
        HasAddIdentitySet<R> +
        HasMulIdentitySet<R> +
        std::ops::Mul<R,Output=R> +
        std::ops::Add<R,Output=R> +
        std::ops::Sub<R, Output=R> +
        std::ops::Neg
{}

pub trait ModuleSet<R,U=Self>
where
    U:
        AbelianGroupSet<U> +
        AbelianMonoidSet<U> +
        AbelianSemiGroupSet<U> +
        HasAddIdentitySet<U> +
        std::ops::Mul<U,Output=U> +
        std::ops::Add<U,Output=U> +
        std::ops::Sub<U, Output=U> +
        std::ops::Neg +
        std::ops::Mul<R,Output=U>,
    R:
        RingSet<R> +
        RngSet<R> +
        CommutativeMonoidSet<R> +
        MonoidSet<R> +
        SemiGroupSet<R> +
        AbelianGroupSet<R> +
        AbelianMonoidSet<R> +
        AbelianSemiGroupSet<R> +
        HasAddIdentitySet<R> +
        HasMulIdentitySet<R> +
        std::ops::Mul<R,Output=R> +
        std::ops::Add<R,Output=R> +
        std::ops::Sub<R, Output=R> +
        std::ops::Neg
{}