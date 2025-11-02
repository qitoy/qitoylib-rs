//! モンゴメリ乗算

use std::ops::{Add, Mul, Neg, Sub};

/// モンゴメリ乗算で内部的に使われる型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mvalue {
    val: u128,
    m: Montgomery,
}

impl Mvalue {
    /// 保有している数値を取り出す。
    pub fn val(&self) -> u64 {
        self.m.reduction(self.val) as u64
    }

    pub fn pow(mut self, mut rhs: u64) -> Self {
        let mut r = self.m.trans(1);
        while rhs > 0 {
            if rhs & 1 == 1 {
                r = r * self.clone();
            }
            self = self.clone() * self;
            rhs >>= 1;
        }
        r
    }
}

impl Add for Mvalue {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.m, rhs.m);
        let val = self.val + rhs.val;
        let m = self.m.m;
        let val = if val >= m { val - m } else { val };
        Self { val, m: self.m }
    }
}

impl Neg for Mvalue {
    type Output = Self;
    fn neg(self) -> Self::Output {
        if self.val == 0 {
            self
        } else {
            Self {
                val: self.m.m - self.val,
                m: self.m,
            }
        }
    }
}

impl Sub for Mvalue {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for Mvalue {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.m, rhs.m);
        let val = self.m.reduction(self.val * rhs.val);
        Self { val, m: self.m }
    }
}

/// モンゴメリ乗算をするための構造体。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Montgomery {
    r: u128,
    m: u128,
}

impl Montgomery {
    /// `modulus`で前計算をする。ただし`modulus`は奇数でなくてはならない。
    pub fn new(modulus: u64) -> Self {
        assert!(modulus % 2 != 0);
        let m = modulus.into();
        let (mut r, mut t) = (0, 0);
        for i in 0..64 {
            if t & 1 == 0 {
                t += m;
                r += 1 << i;
            }
            t >>= 1;
        }
        Self { r, m }
    }

    /// `val`をモンゴメリ乗算用に変換する。
    pub fn trans(&self, val: u64) -> Mvalue {
        let val: u128 = val.into();
        let val = (val << 64) % self.m;
        Mvalue {
            val,
            m: self.clone(),
        }
    }

    fn reduction(&self, val: u128) -> u128 {
        let mask: u128 = u64::MAX.into();
        let b = ((val & mask) * self.r) & mask;
        let c = (val + b * self.m) >> 64;
        if c >= self.m { c - self.m } else { c }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let m = Montgomery::new(13);
        let (a, b) = (m.trans(6), m.trans(9));
        let c = a + b;
        assert_eq!(c.val(), 2);
    }

    #[test]
    fn mul() {
        let m = Montgomery::new(13);
        let (a, b) = (m.trans(6), m.trans(9));
        let c = a * b;
        assert_eq!(c.val(), 2);
    }

    #[test]
    fn pow() {
        let m = Montgomery::new(998244353);
        let a = m.trans(114514).pow(1919810);
        assert_eq!(a.val(), 306961278);
    }
}
