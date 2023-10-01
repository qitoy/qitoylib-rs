extern crate qitoy_data_structure_bit_vec;
use qitoy_data_structure_bit_vec::BitVec;

#[derive(Debug)]
pub struct WaveletMatrix {
    matrix: Vec<BitVec>,
}

impl From<Vec<u64>> for WaveletMatrix {
    fn from(mut value: Vec<u64>) -> Self {
        let mut matrix = vec![];
        for i in (0..64).rev() {
            let vec: Vec<_> = value.iter().map(|v| v >> i & 1).collect();
            matrix.push(vec.into());
            value.sort_by_key(|v| v >> i & 1);
        }
        Self { matrix }
    }
}

impl WaveletMatrix {
    pub fn access(&self, mut n: usize) -> u64 {
        let mut ret = 0;
        for i in 0..64 {
            let bitvec = &self.matrix[i];
            if bitvec.at(n) {
                let zeros = bitvec.rank0(bitvec.len());
                n = (zeros + bitvec.rank1(n)) as usize;
                ret |= 1 << (63 - i);
            } else {
                n = bitvec.rank0(n) as usize;
            }
        }
        ret
    }
    pub fn rank(&self, val: u64, n: usize) -> u32 {
        (self.rank_pos(val, n) - self.rank_pos(val, 0)) as u32
    }
    fn rank_pos(&self, val: u64, mut n: usize) -> usize {
        for i in 0..64 {
            let bitvec = &self.matrix[i];
            if val >> (63 - i) & 1 == 1 {
                let zeros = bitvec.rank0(bitvec.len());
                n = (zeros + bitvec.rank1(n)) as usize;
            } else {
                n = bitvec.rank0(n) as usize;
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed() {
        let vec = vec![5, 4, 5, 5, 2, 1, 5, 6, 1, 3, 5, 0];
        let wm = WaveletMatrix::from(vec.to_vec());
        // 4cf, 409, 5eb
        // println!("{:?}", wm);
        for (i, &v) in vec.iter().enumerate() {
            assert_eq!(v, wm.access(i));
        }
        assert_eq!(wm.rank(5, 9), 4);
        assert_eq!(wm.rank(1, 8), 1);
    }
}
