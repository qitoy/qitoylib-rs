use std::{
    convert::Infallible,
    marker::PhantomData,
    ops::{Add, Mul, Neg},
};

pub trait Ring: Clone {
    type S: Clone;
    fn add(a: &Self::S, b: &Self::S) -> Self::S;
    fn neg(a: &Self::S) -> Self::S;
    fn mul(a: &Self::S, b: &Self::S) -> Self::S;
    fn zero() -> Self::S;
    fn one() -> Self::S;
}

#[derive(Clone)]
pub struct AddMul<T>(Infallible, PhantomData<fn() -> T>);

impl<T> Ring for AddMul<T>
where
    T: Clone + From<u8> + Add<Output = T> + Neg<Output = T> + Mul<Output = T>,
{
    type S = T;
    fn add(a: &Self::S, b: &Self::S) -> Self::S {
        a.clone() + b.clone()
    }
    fn neg(a: &Self::S) -> Self::S {
        -a.clone()
    }
    fn mul(a: &Self::S, b: &Self::S) -> Self::S {
        a.clone() * b.clone()
    }
    fn zero() -> Self::S {
        0.into()
    }
    fn one() -> Self::S {
        1.into()
    }
}

#[derive(Clone)]
pub struct XorAnd<T>(Infallible, PhantomData<fn() -> T>);

impl Ring for XorAnd<u64> {
    type S = u64;
    fn add(a: &Self::S, b: &Self::S) -> Self::S {
        a ^ b
    }
    fn neg(a: &Self::S) -> Self::S {
        *a
    }
    fn mul(a: &Self::S, b: &Self::S) -> Self::S {
        a * b
    }
    fn zero() -> Self::S {
        0
    }
    fn one() -> Self::S {
        u64::MAX
    }
}
