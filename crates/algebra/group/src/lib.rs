use ac_library::{Additive, Monoid};
use std::ops::Neg;

pub trait Group: Monoid {
    fn inverse(a: &Self::S) -> Self::S;
}

impl<S> Group for Additive<S>
where
    S: Neg<Output = S> + Copy,
    Additive<S>: Monoid<S = S>,
{
    fn inverse(a: &Self::S) -> Self::S {
        -*a
    }
}

pub trait Abelian: Group {}
