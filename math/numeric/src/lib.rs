extern crate num;

use std::ops::{Mul, Div, Rem};
use num::traits::Zero;

pub trait Gcd: Rem<Output = Self> + Zero + PartialEq + Copy {
    fn gcd(self, rhs: Self) -> Self;
}

pub trait Lcm: Gcd + Mul<Output = Self> + Div<Output = Self> {
    fn lcm(self, rhs: Self) -> Self;
}

/// gcdを計算します。  
/// 計算量$`O(\log \max(self, rhs))`$
impl<T: Rem<Output = Self> + Zero + PartialEq + Copy> Gcd for T {
    fn gcd(mut self, mut rhs: Self) -> Self {
        while rhs != T::zero() {
            (self, rhs) = (rhs, self % rhs);
        }
        self
    }
}

/// lcmを計算します。  
/// 計算量$`O(\log \max(self, rhs))`$
impl<T: Gcd + Mul<Output = Self> + Div<Output = Self>> Lcm for T {
    fn lcm(self, rhs: Self) -> Self {
        self / self.gcd(rhs) * rhs
    }
}
