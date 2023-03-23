extern crate amplify;
use amplify::num::u1024;

#[derive(Debug)]
pub struct BitVec {
      len: usize,
     data: Vec<u1024>,
    large: Vec<u32>,
    small: Vec<u16>,
}

trait PopCount {
    fn popcount(&self) -> u32;
}

impl PopCount for u1024 {
    fn popcount(&self) -> u32 {
        self.as_inner().iter().fold(0, |acc, x| acc + x.count_ones())
    }
}

impl<T: Into<u1024> + Copy> From<Vec<T>> for BitVec {
    fn from(value: Vec<T>) -> Self {
        let len = (value.len() >> 10) + 1;
        let mut data = vec![u1024::ZERO; len];
        for i in 0..value.len() {
            data[i >> 10] |= value[i].into() << (i & 1023);
        }
        let data = data;
        let large = std::iter::once(&u1024::ZERO).chain(data.iter()).scan(0, |state, &x| {
            *state += x.popcount();
            Some(*state)
        }).take(len).collect();
        let mut small = vec![0; len << 6];
        for i in 0..len {
            for j in 0..63 {
                let idx = i << 6 | j;
                let mask: u1024 = u16::MAX.into();
                small[idx+1] = small[idx] +
                    (data[i] >> 16 * j & mask).popcount() as u16;
            }
        }
        Self { len: value.len(), data, large, small, }
    }
}

fn lower_bound<T: Ord>(arr: &[T], val: T) -> usize {
    let (mut left, mut right) = (0, arr.len());
    while left != right {
        let mid = (left + right) / 2;
        if arr[mid] >= val { right = mid; } else { left = mid + 1; }
    }
    left
}

impl BitVec {
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn at(&self, n: usize) -> bool {
        self.data[n >> 10].bit(n & 1023)
    }
    pub fn rank1(&self, n: usize) -> u32 {
        assert!(n <= self.len);
        let m = n & 1023;
        let mask = (u1024::ONE << m) - (u1024::ONE << (m >> 4 << 4));
        self.large[n >> 10]
            + self.small[n >> 4] as u32
            + (self.data[n >> 10] & mask).popcount()
    }
    pub fn rank0(&self, n: usize) -> u32 {
        n as u32 - self.rank1(n)
    }
    pub fn select1(&self, n: u32) -> Option<usize> {
        if n == 0 { return Some(0); }
        let i = lower_bound(&self.large, n) - 1;
        let n = (n - self.large[i]) as u16;
        let j = lower_bound(&self.small[i << 6 .. i+1 << 6], n) - 1;
        let mut n = n - self.small[i << 6 | j];
        for k in 0..16 {
            let k = i << 10 | j << 4 | k;
            if n == 0 { return Some(k); }
            if self.at(k) { n -= 1; }
        }
        if n == 0 { Some((i << 10 | j << 4) + 16) } else { None }
    }
    pub fn select0(&self, n: u32) -> Option<usize> {
        if let Some(r) = self._select0(n) {
            if r <= self.len { Some(r) } else { None }
        } else { None }
    }
    fn _select0(&self, n: u32) -> Option<usize> {
        if n == 0 { return Some(0); }
        let i = {
            let arr = &self.large;
            let (mut left, mut right) = (0, arr.len());
            while left != right {
                let mid = (left + right) / 2;
                if ((mid as u32) << 10) - arr[mid] >= n { right = mid; } else { left = mid + 1; }
            }
            left
        } - 1;
        let n = (n + self.large[i] - ((i as u32) << 10)) as u16;
        let j = {
            let arr = &self.small[i << 6 .. i+1 << 6];
            let (mut left, mut right) = (0, arr.len());
            while left != right {
                let mid = (left + right) / 2;
                if ((mid as u16) << 4) - arr[mid] >= n { right = mid; } else { left = mid + 1; }
            }
            left
        } - 1;
        let mut n = n + self.small[i << 6 | j] - ((j as u16) << 4);
        for k in 0..16 {
            let k = i << 10 | j << 4 | k;
            if n == 0 { return Some(k); }
            if !self.at(k) { n -= 1; }
        }
        if n == 0 { Some((i << 10 | j << 4) + 16) } else { None }
    }
}

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    use rand::distributions::Uniform;

    #[test]
    fn print() {
        let bv = BitVec::from(vec![1u32; 1030]);
        eprintln!("{:?}", bv);
        eprintln!("{}", bv.rank1(1030));
        eprintln!("{:?}", bv.select1(100));
        eprintln!("{:?}", bv.select1(10000));
    }

    #[test]
    fn bitvec() {
        let mut rng = thread_rng();
        let vec: Vec<u32> = (&mut rng).sample_iter(Uniform::from(0..=1)).take(10000).collect();
        let bv = BitVec::from(vec);
        let mut rank1 = vec![0; 10001];
        let mut rank0 = vec![0; 10001];
        let mut select1 = vec![0];
        let mut select0 = vec![0];
        for i in 0..10000 {
            if bv.at(i) {
                rank1[i+1] = rank1[i] + 1;
                rank0[i+1] = rank0[i];
                select1.push(i+1);
            } else {
                rank1[i+1] = rank1[i];
                rank0[i+1] = rank0[i] + 1;
                select0.push(i+1);
            }
        }
        for _ in 0..10000 {
            let r: usize = (&mut rng).sample(Uniform::from(0..10000));
            eprintln!("test for rank1");
            assert_eq!(rank1[r], bv.rank1(r));
            eprintln!("test for rank0");
            assert_eq!(rank0[r], bv.rank0(r));
            // eprintln!("rank({}) is {}", r, bv.rank1(r));
            // eprintln!("select({}) is {:?}", r, bv.select1(r as u32));
            eprintln!("test for select1");
            if let Some(idx) = bv.select1(r as u32) {
                assert_eq!(select1[r], idx);
            } else {
                assert_eq!(select1.get(r), None);
            }
            eprintln!("test for select0");
            if let Some(idx) = bv.select0(r as u32) {
                assert_eq!(select0[r], idx);
            } else {
                assert_eq!(select0.get(r), None);
            }
        }
    }
}
