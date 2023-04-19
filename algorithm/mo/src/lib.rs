//! Mo's algorithmを提供するトレイトです。  

use std::convert::TryInto;

pub trait Mo {
    type T;

    fn push_left(&mut self, val: &mut Self::T, idx: usize);

    fn pop_left(&mut self, val: &mut Self::T, idx: usize);

    fn push_right(&mut self, val: &mut Self::T, idx: usize);

    fn pop_right(&mut self, val: &mut Self::T, idx: usize);
    
    /// * `val` - クエリに対応する値
    /// * `idx` - クエリの何番目か
    fn apply(&mut self, val: &Self::T, idx: usize);

    fn solve(&mut self, query: &[(usize, usize)], init: Self::T) {
        let k = query.iter().map(|x| x.1).max().unwrap();
        let k: usize = (k.ilog2()+1).try_into().unwrap();
        let mut query: Vec<_> = query.into_iter().zip(0..).collect();
        query.sort_by_cached_key(|x| {
            let (l, r) = *x.0;
            hilbert_order(k, l, r)
        });
        let mut val = init;
        let (mut l, mut r) = (0, 0);
        for (&(nl, nr), idx) in query.into_iter() {
            while r < nr { self.push_right(&mut val, r); r += 1; }
            while nl < l { l -= 1; self.push_left(&mut val, l); }
            while nr < r { r -= 1; self.pop_right(&mut val, r); }
            while l < nl { self.pop_left(&mut val, l); l += 1; }
            self.apply(&val, idx);
        }
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
