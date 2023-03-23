extern crate qitoy_prime_check;
extern crate qitoy_math_montgomery;
extern crate qitoy_math_numeric;
extern crate num;
extern crate itertools;
extern crate rand;

use qitoy_prime_check::PrimeCheck;
use qitoy_math_montgomery::*;
use qitoy_math_numeric::Gcd;
use std::ops::Div;
use rand::{SeedableRng, rngs::SmallRng, distributions::{Distribution, Uniform}};
use num::traits::One;
use itertools::Itertools;

pub trait Factorise: PrimeCheck {
    /// `self`が合成数のとき非自明な素因数を一つ返す。
    fn find_factor(self) -> Self;

    /// `self`を素因数分解する。結果はソートされる。
    fn factorise(self) -> Vec<Self>
    where Self: Div<Output = Self> + One + Copy + PartialOrd
    {
        let n = self;
        if n == Self::one() { return vec![]; }
        if n.prime_check() { return vec![self]; }
        let d = n.find_factor();
        let r = d.factorise();
        r.into_iter().merge((n/d).factorise().into_iter()).collect()
    }
}

impl Factorise for u64 {
    fn find_factor(self) -> Self {
        let n = self;
        if n & 1 == 0 { return 2; }
        let mut g;
        let mut rng = SmallRng::from_entropy();
        let range = Uniform::from(1..n);
        let mo = Montgomery::new(n);
        while {
            let (mut x, mut ys);
            let (y, c) = (range.sample(&mut rng), range.sample(&mut rng));
            let (mut y, c, mut q) = (mo.trans(y), mo.trans(c), mo.trans(1));
            let mut r = 1;
            let m = 128;
            let f = |x: Mvalue| {
                x.clone() * x + c.clone()
            };
            while {
                x = y.clone();
                for _ in 0..r { y = f(y); }
                let mut k = 0;
                while {
                    ys = y.clone();
                    for _ in 0..m.min(r-k) {
                        y = f(y);
                        q = q * (x.clone() - y.clone());
                    }
                    g = q.val().gcd(n); k += m;
                    k < r && g == 1
                } {}
                r <<= 1;
                g == 1
            } {}
            if g == n {
                while {
                    ys = f(ys);
                    g = (x.clone() - ys.clone()).val().gcd(n);
                    g == 1
                } {}
            }
            g == n
        } {}
        g
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4033_4681() {
        let v = 4033.factorise();
        assert_eq!(v, vec![37, 109]);
        let v = 4681.factorise();
        assert_eq!(v, vec![31, 151]);
    }
    #[test]
    fn test_4295098369() {
        let v = 4295098369.factorise();
        assert_eq!(v, vec![65537, 65537]);
    }
    #[test]
    fn test_999381247093216751() {
        let v = 999381247093216751.factorise();
        assert_eq!(v, vec![999665081, 999716071]);
    }
    #[test]
    fn test_124376107291() {
        let v = 124376107291.factorise();
        assert_eq!(v, vec![352523, 352817]);
    }

    #[test]
    fn test_897612484786617600() {
        let v = 897612484786617600.factorise();
        assert_eq!(v, vec![
                   2, 2, 2, 2, 2, 2, 2, 2,
                   3, 3, 3, 3, 5, 5, 7, 7,
                   11, 13, 17, 19, 23, 29, 31, 37,
        ]);
    }
}
