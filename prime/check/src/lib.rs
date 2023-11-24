use qitoy_math_montgomery::Montgomery;

pub trait PrimeCheck {
    /// 素数判定をする。
    fn prime_check(&self) -> bool;
}

impl PrimeCheck for u64 {
    fn prime_check(&self) -> bool {
        let &n = self;
        if n <= 2 {
            return n == 2;
        }
        if n & 1 == 0 {
            return false;
        }
        let r = (n - 1).trailing_zeros();
        let d = (n - 1) >> r;
        let m = Montgomery::new(n);
        let (one, n1) = (m.trans(1), m.trans(n - 1));
        for a in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
            if a % n == 0 {
                return true;
            }
            let mut x = m.trans(a).pow(d);
            if x == one || x == n1 {
                continue;
            }
            for i in 0..r {
                x = x.clone() * x;
                if x == n1 {
                    break;
                }
                if x == one || i == r - 1 {
                    return false;
                }
            }
        }
        true
    }
}
