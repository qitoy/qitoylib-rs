use std::convert::TryInto;

pub struct Mo<PushLeft, PopLeft, PushRight, PopRight, Assign> {
    pub push_left: PushLeft,
    pub pop_left: PopLeft,
    pub push_right: PushRight,
    pub pop_right: PopRight,
    pub assign: Assign,
}

impl<PushLeft, PopLeft, PushRight, PopRight, Assign>
    Mo<PushLeft, PopLeft, PushRight, PopRight, Assign>
{
    pub fn solve<T>(&mut self, query: &[(usize, usize)], init: T)
    where
        PushLeft: FnMut(&mut T, usize),
        PopLeft: FnMut(&mut T, usize),
        PushRight: FnMut(&mut T, usize),
        PopRight: FnMut(&mut T, usize),
        Assign: FnMut(&T, usize),
    {
        let k = query.iter().map(|x| x.1).max().unwrap();
        let k: usize = (k.ilog2() + 1).try_into().unwrap();
        let mut query: Vec<_> = query.iter().zip(0..).collect();
        query.sort_by_cached_key(|x| {
            let (l, r) = *x.0;
            hilbert_order(k, l, r)
        });
        let mut val = init;
        let (mut l, mut r) = (0, 0);
        for (&(nl, nr), idx) in query.into_iter() {
            while r < nr {
                (self.push_right)(&mut val, r);
                r += 1;
            }
            while nl < l {
                l -= 1;
                (self.push_left)(&mut val, l);
            }
            while nr < r {
                r -= 1;
                (self.pop_right)(&mut val, r);
            }
            while l < nl {
                (self.pop_left)(&mut val, l);
                l += 1;
            }
            (self.assign)(&val, idx);
        }
    }
}

fn hilbert_order(k: usize, x: usize, y: usize) -> u64 {
    let (mut x, mut y): (u64, u64) = (x.try_into().unwrap(), y.try_into().unwrap());
    let mut d = 0;
    for i in (0..k).rev() {
        let (rx, ry) = (x >> i & 1, y >> i & 1);
        d += (1 << (2 * i)) * ((3 * rx) ^ ry);
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
