pub trait PrimeCheck {
    fn prime_check(self) -> bool;
}

fn mod_pow(a: u64, mut n: u64, m: u64) -> u64 {
    let (mut a, m): (u128, u128) = (a.into(), m.into());
    let mut r = 1;
    while n > 0 {
        if n & 1 == 1 { r = r * a % m; }
        a = a * a % m;
        n >>= 1;
    }
    r.try_into().unwrap()
}

impl PrimeCheck for u64 {
    fn prime_check(self) -> bool {
        let n = self;
        if n <= 2 { return n == 2; }
        if n & 1 == 0 { return false; }
        let r = (n-1).trailing_zeros();
        let d = n-1 >> r;
        for a in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
            if a % n == 0 { return true; }
            let mut x = mod_pow(a, d, n);
            if x == 1 || x == n-1 { continue; }
            for i in 0..r {
                x = mod_pow(x, 2, n);
                if x == n-1 { break; }
                if x == 1 || i == r-1 { return false; }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::PrimeCheck;

    #[test]
    fn test() {
        assert!(2.prime_check());
        assert!(7.prime_check());
        assert!(1_000_000_007.prime_check());
        assert!(998_244_353.prime_check());
        assert!(65537.prime_check());
        assert!(!10.prime_check());
        assert!(!4033.prime_check());
        assert!(!4681.prime_check());
    }
}
