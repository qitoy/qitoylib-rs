use std::ops::{Add, Mul, Neg};

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

extern crate ac_library;
use ac_library::modint::{ModInt, ModInt1000000007, ModInt998244353};
impl_ring_mint!(ModInt, ModInt998244353, ModInt1000000007);

#[derive(Clone)]
struct XorAnd(u64);

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<Self> for XorAnd {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Neg for XorAnd {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul<Self> for XorAnd {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Ring for XorAnd {
    fn one() -> Self {
        Self(u64::MAX)
    }
    fn zero() -> Self {
        Self(0)
    }
}
