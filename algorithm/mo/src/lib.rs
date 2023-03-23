//! Mo's algorithmを提供するトレイトです。  

use std::convert::TryInto;

/// 実装が必要な4つの関数は、範囲を伸ばし縮みさせたときに呼ばれ、範囲からpush/popされたindexを用いて`val`を変更します。
pub trait Mo {
    type T: Default + Copy;

    /// 範囲の左側を伸ばしたときに呼ばれます。
    fn push_left(&mut self, val: &mut Self::T, idx: usize);

    /// 範囲の左側を縮めたときに呼ばれます。
    fn pop_left(&mut self, val: &mut Self::T, idx: usize);

    /// 範囲の右側を伸ばしたときに呼ばれます。
    fn push_right(&mut self, val: &mut Self::T, idx: usize);

    /// 範囲の右側を縮めたときに呼ばれます。
    fn pop_right(&mut self, val: &mut Self::T, idx: usize);

    /// 複数の範囲クエリに対して、それに対応する値を格納したベクタを返します。
    fn solve(&mut self, query: &[(usize, usize)]) -> Vec<Self::T> {
        let k = query.iter().map(|x| x.1).max().unwrap();
        let k: usize = (k.ilog2()+1).try_into().unwrap();
        let mut query: Vec<_> = query.into_iter().zip(0..).collect();
        query.sort_by_cached_key(|x| {
            let (l, r) = *x.0;
            hilbert_order(k, l, r)
        });
        let mut ret = vec![Self::T::default(); query.len()];
        let mut val = Self::T::default();
        let (mut l, mut r) = (0, 0);
        for (&(nl, nr), i) in query.into_iter() {
            while r < nr { self.push_right(&mut val, r); r += 1; }
            while nl < l { l -= 1; self.push_left(&mut val, l); }
            while nr < r { r -= 1; self.pop_right(&mut val, r); }
            while l < nl { self.pop_left(&mut val, l); l += 1; }
            ret[i] = val;
        }
        ret
    }
}

fn hilbert_order(k: usize, x: usize, y: usize) -> u64 {
    let (mut x, mut y): (u64, u64) = (x.try_into().unwrap(), y.try_into().unwrap());
    let mut d = 0;
    for i in (0..k).rev() {
        let (rx, ry) = (x >> i & 1, y >> i & 1);
        d += (1 << 2 * i) * (3 * rx ^ ry);
        if ry == 0 {
            if rx == 1 {
                x = (1 << k) - 1 - x;
                y = (1 << k) - 1 - y;
            }
            (x, y) = (y, x);
        }
    }
    d
}
