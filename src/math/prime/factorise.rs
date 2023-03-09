use super::check::PrimeCheck;
use rand::{SeedableRng, rngs::SmallRng, distributions::{Distribution, Uniform}};
use num::integer::gcd;

trait Factorise
where Self: Sized {
    fn find_factor(self) -> Self;
    fn factorise(self) -> Vec<Self>;
}

impl Factorise for u64 {
    fn find_factor(self) -> Self {
        let n: u128 = self.into();
        if n & 1 == 0 { return 2; }
        let mut g;
        let mut rng = SmallRng::from_entropy();
        let range = Uniform::from(1..n);
        while {
            let (mut x, mut ys, mut k);
            let (mut y, c) = (range.sample(&mut rng), range.sample(&mut rng));
            let (mut r, mut q) = (1, 1);
            let m = 128;
            let f = |x| {
                (x * x + c) % n
            };
            while {
                x = y;
                for _ in 0..r { y = f(y); }
                k = 0;
                while {
                    ys = y;
                    for _ in 0..m.min(r-k) {
                        y = f(y);
                        q = (q * x.abs_diff(y) % n).try_into().unwrap();
                    }
                    g = gcd(q, n); k += m;
                    k < r && g == 1
                } {}
                r <<= 1;
                g == 1
            } {}
            if g == n {
                while {
                    ys = f(ys);
                    g = gcd(x.abs_diff(ys), n);
                    g == 1
                } {}
            }
            g == n
        } {}
        g.try_into().unwrap()
    }
    fn factorise(self) -> Vec<Self> {
        let n = self;
        if n == 1 { return vec![]; }
        if n.prime_check() { return vec![self]; }
        let d = n.find_factor();
        let mut r = d.factorise();
        r.append(&mut (n/d).factorise());
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4033_4681() {
        let mut v = 4033.factorise();
        v.sort();
        assert_eq!(v, vec![37, 109]);
        let mut v = 4681.factorise();
        v.sort();
        assert_eq!(v, vec![31, 151]);
    }
    #[test]
    fn test_4295098369() {
        let mut v = 4295098369.factorise();
        v.sort();
        assert_eq!(v, vec![65537, 65537]);
    }
    #[test]
    fn test_999381247093216751() {
        let mut v = 999381247093216751.factorise();
        v.sort();
        assert_eq!(v, vec![999665081, 999716071]);
    }
    #[test]
    fn test_124376107291() {
        let mut v = 124376107291.factorise();
        v.sort();
        assert_eq!(v, vec![352523, 352817]);
    }
}
