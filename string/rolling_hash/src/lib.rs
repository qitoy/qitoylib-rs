//! ローリングハッシュ

extern crate once_cell;
extern crate rand;
use ac_library::Monoid;
use once_cell::sync::Lazy;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::SmallRng,
    SeedableRng,
};

/// ロリハ
/// [`Monoid`](ac_library::Monoid)として使う。
pub struct RollingHash;

impl RollingHash {
    const MOD: u64 = (1 << 61) - 1;
    const ROOT: u64 = 37;
    const CHARSIZE: u64 = 256;

    /// 文字`c`からロリハを生成する。
    pub fn new(c: char) -> <Self as Monoid>::S {
        let (b1, b2) = Self::bases();
        (b1, b2, c as u64, c as u64)
    }

    /// https://trap.jp/post/1036/
    fn bases() -> (u64, u64) {
        static BASES: Lazy<(u64, u64)> = Lazy::new(|| {
            let gcd = |mut x, mut y| {
                while y > 0 {
                    (x, y) = (y, x % y)
                }
                x
            };
            let mut rng = SmallRng::from_entropy();
            let range = Uniform::from(0..RollingHash::MOD);
            let mut base = |b| loop {
                let k = range.sample(&mut rng);
                if gcd(k, RollingHash::MOD - 1) != 1 {
                    continue;
                }
                let r = RollingHash::pow(RollingHash::ROOT, k);
                if r <= RollingHash::CHARSIZE || r == b {
                    continue;
                }
                return r;
            };
            let b1 = base(0);
            (b1, base(b1))
        });
        *BASES
    }

    fn add(a: u64, b: u64) -> u64 {
        let c = a + b;
        if c >= Self::MOD {
            c - Self::MOD
        } else {
            c
        }
    }

    fn neg(a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            Self::MOD - a
        }
    }

    fn sub(a: u64, b: u64) -> u64 {
        Self::add(a, Self::neg(b))
    }

    fn mul(a: u64, b: u64) -> u64 {
        let c = (a as u128) * (b as u128);
        let m = Self::MOD as u128;
        let c = (c >> 61) + (c & m);
        (if c >= m { c - m } else { c }) as u64
    }

    fn pow(mut a: u64, mut b: u64) -> u64 {
        let mut c = 1;
        while b > 0 {
            if b % 2 == 1 {
                c = Self::mul(c, a);
            }
            a = Self::mul(a, a);
            b /= 2;
        }
        c
    }
}

impl Monoid for RollingHash {
    /// (base1, base2, hash1, hash2)
    type S = (u64, u64, u64, u64);
    fn identity() -> Self::S {
        (1, 1, 0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (
            Self::mul(a.0, b.0),
            Self::mul(a.1, b.1),
            Self::add(Self::mul(a.2, b.0), b.2),
            Self::add(Self::mul(a.3, b.1), b.3),
        )
    }
}

/// ロリハによる文字列検索のための構造体。
/// ```
/// use qitoy_rolling_hash::RhVec;
/// let s = "abracadabra";
/// let h: RhVec = s.chars().collect();
/// assert_eq!(h.get(0..4), h.get(7..11)); // "abra"
/// assert_eq!(h.get(0..0), h.get(4..4)); // ""
/// assert_ne!(h.get(0..4), h.get(4..8)); // "abra", "cada"
/// ```
pub struct RhVec {
    data: Vec<<RollingHash as Monoid>::S>,
}

impl RhVec {
    pub fn get(&self, range: std::ops::Range<usize>) -> <RollingHash as Monoid>::S {
        let (l, r) = (range.start, range.end);
        let b = self.data[r - l];
        let (l, r) = (self.data[l], self.data[r]);
        (
            b.0,
            b.1,
            RollingHash::sub(r.2, RollingHash::mul(l.2, b.0)),
            RollingHash::sub(r.3, RollingHash::mul(l.3, b.1)),
        )
    }
}

impl FromIterator<char> for RhVec {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let e = RollingHash::identity();
        let data = std::iter::once(e)
            .chain(iter.into_iter().scan(e, |state, x| {
                *state = RollingHash::binary_operation(state, &RollingHash::new(x));
                Some(*state)
            }))
            .collect();
        Self { data }
    }
}
