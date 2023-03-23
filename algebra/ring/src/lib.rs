use std::ops::{Add, Neg, Mul};

pub trait Ring: Clone + Add<Output = Self> + Neg<Output = Self> + Mul<Output = Self> {
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! impl_ring_int {
    ($( $ty:ty ),*) => {
        $(
            impl Ring for $ty {
                fn one() -> Self { 1 }
                fn zero() -> Self { 0 }
            }
        )*
    }
}

impl_ring_int!(i32, i64);

macro_rules! impl_ring_mint {
    ($( $ty:ty ),*) => {
        $(
            impl Ring for $ty {
                fn one() -> Self { Self::raw(1) }
                fn zero() -> Self { Self::raw(0) }
            }
        )*
    }
}

extern crate ac_library_rs;
use ac_library_rs::modint::{ModInt, ModInt998244353, ModInt1000000007};
impl_ring_mint!(ModInt, ModInt998244353, ModInt1000000007);
