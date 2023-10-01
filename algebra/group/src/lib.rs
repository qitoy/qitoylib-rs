use ac_library::{Additive, Monoid};
use std::ops::Neg;

pub trait Group: Monoid {
    fn negate(a: &Self::S) -> Self::S;
}

impl<S> Group for Additive<S>
where
    S: Neg<Output = S> + Copy,
    Additive<S>: Monoid<S = S>,
{
    fn negate(a: &Self::S) -> Self::S {
        -*a
    }
}

trait Abelian: Group {}

