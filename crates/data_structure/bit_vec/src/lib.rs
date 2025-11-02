use qitoy_bit_util::BitUtil;

/// コンパクト辞書
pub struct BitVec {
    len: usize,
    data: Vec<u64>,
    block: Vec<u32>,
}

impl std::fmt::Debug for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data: String = self
            .data
            .iter()
            .enumerate()
            .flat_map(|(i, d)| {
                format!("{d:064b}")
                    .chars()
                    .rev()
                    .take(self.len - i * 64)
                    .chain([' '])
                    .collect::<Vec<_>>()
            })
            .collect();
        f.debug_struct("BitVec")
            .field("len", &self.len)
            .field("data", &data)
            .field("block", &self.block)
            .finish()
    }
}

impl<I: Into<u64>> FromIterator<I> for BitVec {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let v: Vec<u64> = iter.into_iter().map(Into::into).collect();
        let len = v.len();
        let mut data = vec![0u64; (len + 63) / 64];
        let mut block = vec![0; (len + 63) / 64];
        for i in v
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| if v == 1 { Some(i) } else { None })
        {
            data[i / 64].bit_set(i % 64);
        }
        for (i, &d) in data.iter().enumerate().take(data.len() - 1) {
            block[i + 1] = block[i] + d.count_ones();
        }
        Self { len, data, block }
    }
}

impl BitVec {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn at(&self, p: usize) -> bool {
        self.data[p / 64].bit_get(p % 64)
    }

    pub fn rank1(&self, p: usize) -> usize {
        (self.block[p / 64] + masked(self.data[p / 64], p % 64).count_ones()) as usize
    }

    pub fn rank0(&self, p: usize) -> usize {
        p - self.rank1(p)
    }

    pub fn select1(&self, n: usize) -> Option<usize> {
        if n == 0 {
            return Some(0);
        }
        let n = n as u32;
        let idx = self.block.partition_point(|&b| b < n) - 1;
        let n = n - self.block[idx];
        (0..64)
            .filter(|i| self.data[idx].bit_get(i))
            .nth(n as usize - 1)
            .map(|i| idx * 64 + i)
    }

    pub fn select0(&self, n: usize) -> Option<usize> {
        if n == 0 {
            return Some(0);
        }
        let n = n as u32;
        let idx = {
            let (mut l, mut r) = (0, self.block.len());
            while r - l > 1 {
                let mid = (l + r) / 2;
                if mid as u32 * 64 - self.block[mid] < n {
                    l = mid;
                } else {
                    r = mid;
                }
            }
            l
        };
        let n = n - (idx as u32 * 64 - self.block[idx]);
        (0..64)
            .filter(|i| !self.data[idx].bit_get(i))
            .nth(n as usize - 1)
            .map(|i| idx * 64 + i)
            .filter(|&i| i < self.len)
    }
}

fn masked(bit: u64, len: usize) -> u64 {
    bit & ((1 << len) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Uniform;
    use rand::{Rng, thread_rng};

    #[test]
    fn print() {
        let bv: BitVec = std::iter::repeat(1u32).take(1030).collect();
        eprintln!("{:?}", bv);
        eprintln!("{}", bv.rank1(1030));
        eprintln!("{:?}", bv.select1(100));
        eprintln!("{:?}", bv.select1(10000));
    }

    #[test]
    fn bitvec() {
        let mut rng = thread_rng();
        let bv: BitVec = (&mut rng)
            .sample_iter(Uniform::from(0u32..=1))
            .take(10000)
            .collect();
        eprintln!("{bv:?}");
        let mut rank1 = vec![0; 10001];
        let mut rank0 = vec![0; 10001];
        let mut select1 = vec![0];
        let mut select0 = vec![0];
        for i in 0..10000 {
            if bv.at(i) {
                rank1[i + 1] = rank1[i] + 1;
                rank0[i + 1] = rank0[i];
                select1.push(i);
            } else {
                rank1[i + 1] = rank1[i];
                rank0[i + 1] = rank0[i] + 1;
                select0.push(i);
            }
        }
        for r in 0..10000 {
            assert_eq!(bv.rank1(r), rank1[r], "rank1({r})");
            assert_eq!(bv.rank0(r), rank0[r], "rank0({r})");
            // eprintln!("rank({}) is {}", r, bv.rank1(r));
            // eprintln!("select({}) is {:?}", r, bv.select1(r as u32));
            assert_eq!(bv.select1(r), select1.get(r).copied(), "select1({r})");
            assert_eq!(bv.select0(r), select0.get(r).copied(), "select0({r})");
        }
    }
}
