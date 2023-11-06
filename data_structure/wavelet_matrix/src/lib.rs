use qitoy_bit_util::BitUtil;
use qitoy_bit_vec::BitVec;

/// Wavelet Matrix
#[derive(Debug)]
pub struct WaveletMatrix {
    matrix: Vec<BitVec>,
    zeros: Vec<usize>,
}

macro_rules! impl_from_iterator {
    ($ty:ty, $e:expr) => {
        impl FromIterator<$ty> for WaveletMatrix {
            fn from_iter<T: IntoIterator<Item = $ty>>(iter: T) -> Self {
                let data = iter.into_iter().map(u64::from).collect();
                Self::new(data, $e)
            }
        }
    };
}

impl_from_iterator!(u32, 32);
impl_from_iterator!(u64, 64);

impl WaveletMatrix {
    /// max data < 1 << size
    pub fn new(mut data: Vec<u64>, size: usize) -> Self {
        let mut matrix = Vec::with_capacity(size);
        let mut zeros = Vec::with_capacity(size);
        for i in (0..size).rev() {
            let bitvec: BitVec = data.iter().map(|v| v.bit_get(i)).collect();
            zeros.push(bitvec.rank0(bitvec.len()));
            matrix.push(bitvec);
            data.sort_by_key(|v| v.bit_get(i));
        }
        Self { matrix, zeros }
    }

    fn size(&self) -> usize {
        self.matrix.len()
    }

    pub fn len(&self) -> usize {
        self.matrix[0].len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn access(&self, mut n: usize) -> u64 {
        let mut ret = 0;
        let size = self.size();
        for i in 0..size {
            let bitvec = &self.matrix[i];
            if bitvec.at(n) {
                n = self.zeros[i] + bitvec.rank1(n);
                ret.bit_set(size - 1 - i);
            } else {
                n = bitvec.rank0(n);
            }
        }
        ret
    }

    pub fn rank(&self, val: u64, n: usize) -> u32 {
        (self.rank_pos(val, n) - self.rank_pos(val, 0)) as u32
    }

    fn rank_pos(&self, val: u64, mut n: usize) -> usize {
        let size = self.size();
        for i in 0..size {
            let bitvec = &self.matrix[i];
            n = if val.bit_get(size - 1 - i) {
                self.zeros[i] + bitvec.rank1(n)
            } else {
                bitvec.rank0(n)
            };
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed() {
        let vec = vec![5u32, 4, 5, 5, 2, 1, 5, 6, 1, 3, 5, 0];
        let wm: WaveletMatrix = vec.iter().copied().collect();
        // 4cf, 409, 5eb
        // println!("{:?}", wm);
        for (i, &v) in vec.iter().enumerate() {
            assert_eq!(v as u64, wm.access(i));
        }
        assert_eq!(wm.rank(5, 9), 4);
        assert_eq!(wm.rank(1, 8), 1);
    }
}
