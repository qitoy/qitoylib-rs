//! ローリングハッシュ

extern crate once_cell;
extern crate rand;
use once_cell::sync::Lazy;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::SmallRng,
    SeedableRng,
};

const MOD: u64 = (1 << 61) - 1;

static BASES: Lazy<(u64, u64)> = Lazy::new(|| {
    let mut rng = SmallRng::from_entropy();
    let range = Uniform::from(129..MOD - 1);
    (range.sample(&mut rng), range.sample(&mut rng))
});

pub struct RollingHash {
    bases: Vec<(u64, u64)>,
    hashes: Vec<(u64, u64)>,
}

impl RollingHash {
    /// `[u8]`からハッシュを前計算します。文字列型に対しては`RollingHash::from()`を使用します。
    pub fn new(str: &[u8]) -> Self {
        let n = str.len();
        let mut bases = vec![(1, 1); n + 1];
        let mut hashes = vec![(0, 0); n + 1];
        for i in 0..n {
            bases[i + 1] = bases[i].rhmul(*BASES);
            hashes[i + 1] = hashes[i]
                .rhmul(*BASES)
                .rhadd((str[i] as u64, str[i] as u64));
        }
        Self { bases, hashes }
    }

    /// `str[l..r]`のハッシュを返します。
    pub fn get_hash(&self, l: usize, r: usize) -> (u64, u64) {
        self.hashes[r].rhsub(self.hashes[l].rhmul(self.bases[r - l]))
    }
}

impl From<&str> for RollingHash {
    fn from(value: &str) -> Self {
        Self::new(value.as_bytes())
    }
}

trait RHHsah {
    fn rhadd(self, rhs: Self) -> Self;
    fn rhsub(self, rhs: Self) -> Self;
    fn rhmul(self, rhs: Self) -> Self;
}

impl RHHsah for u64 {
    fn rhadd(self, rhs: Self) -> Self {
        let r = self + rhs;
        if r >= MOD {
            r - MOD
        } else {
            r
        }
    }
    fn rhsub(self, rhs: Self) -> Self {
        self.rhadd(MOD - rhs)
    }
    fn rhmul(self, rhs: Self) -> Self {
        let t = (self as u128) * (rhs as u128);
        let m = MOD as u128;
        let t = (t >> 61) + (t & m);
        (if t >= m { t - m } else { t }) as u64
    }
}

impl RHHsah for (u64, u64) {
    fn rhadd(self, rhs: Self) -> Self {
        (self.0.rhadd(rhs.0), self.1.rhadd(rhs.1))
    }
    fn rhsub(self, rhs: Self) -> Self {
        (self.0.rhsub(rhs.0), self.1.rhsub(rhs.1))
    }
    fn rhmul(self, rhs: Self) -> Self {
        (self.0.rhmul(rhs.0), self.1.rhmul(rhs.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abracadabra() {
        let s = "abracadabra";
        let h = RollingHash::from(s);
        assert_eq!(h.get_hash(0, 4), h.get_hash(7, 11)); // "abra"
        assert_eq!(h.get_hash(0, 0), h.get_hash(4, 4)); // ""
        assert_ne!(h.get_hash(0, 4), h.get_hash(4, 8)); // "abra", "cada"
    }
}
