extern crate num;
extern crate qitoy_math_montgomery;
extern crate qitoy_prime_check;
extern crate rand;

use num::Integer;
use qitoy_math_montgomery::{Montgomery, Mvalue};
use qitoy_prime_check::PrimeCheck;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::SmallRng,
    SeedableRng,
};

pub struct Factors<T> {
    buf: Vec<T>,
}

impl<T> Iterator for Factors<T>
where
    T: Factorize + Integer + Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.buf.pop()?;
        if n == T::one() {
            return None;
        }
        if n.prime_check() {
            Some(n)
        } else {
            let d = n.find_factor();
            self.buf.append(&mut vec![n / d.clone(), d]);
            self.next()
        }
    }
}

pub trait Factorize: PrimeCheck + Sized {
    /// `self`が合成数のとき非自明な素因数を一つ返す。
    fn find_factor(&self) -> Self;

    /// `self`を素因数分解してイテレータにする。ソートはされない
    fn factorize(self) -> Factors<Self> {
        Factors { buf: vec![self] }
    }
}

impl Factorize for u64 {
    fn find_factor(&self) -> Self {
        let &n = self;
        if n & 1 == 0 {
            return 2;
        }
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
            let f = |x: Mvalue| x.clone() * x + c.clone();
            while {
                x = y.clone();
                for _ in 0..r {
                    y = f(y);
                }
                let mut k = 0;
                while {
                    ys = y.clone();
                    for _ in 0..m.min(r - k) {
                        y = f(y);
                        q = q * (x.clone() - y.clone());
                    }
                    g = q.val().gcd(&n);
                    k += m;
                    k < r && g == 1
                } {}
                r <<= 1;
                g == 1
            } {}
            if g == n {
                while {
                    ys = f(ys);
                    g = (x.clone() - ys.clone()).val().gcd(&n);
                    g == 1
                } {}
            }
            g == n
        } {}
        g
    }
}
