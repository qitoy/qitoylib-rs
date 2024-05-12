use std::ops::{BitAnd, BitOrAssign, Shl, Shr};

pub trait BitUtil<Rhs>:
    From<u8>
    + PartialEq
    + BitAnd<Self, Output = Self>
    + BitOrAssign<Self>
    + Shr<Rhs, Output = Self>
    + Shl<Rhs, Output = Self>
    + Copy
{
    #[inline]
    fn one() -> Self {
        1.into()
    }

    #[inline]
    fn bit_set(&mut self, p: Rhs) {
        *self |= Self::one() << p;
    }

    #[inline]
    fn bit_get(&self, p: Rhs) -> bool {
        *self >> p & Self::one() == Self::one()
    }
}

impl BitUtil<usize> for u32 {}
impl BitUtil<usize> for u64 {}
impl BitUtil<&usize> for u32 {}
impl BitUtil<&usize> for u64 {}
