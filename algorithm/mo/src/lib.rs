pub trait Mo {
    fn push_left(&mut self, l: usize);
    fn pop_left(&mut self, l: usize);
    fn push_right(&mut self, r: usize);
    fn pop_right(&mut self, r: usize);
    fn assign(&mut self, i: usize);

    /// * query - 区間クエリ（開区間）
    fn run(&mut self, query: &[(usize, usize)]) {
        let (mut l, mut r) = (0, 0);
        for idx in noshi91_order(query) {
            let (nl, nr) = query[idx];
            while r < nr {
                self.push_right(r);
                r += 1;
            }
            while nl < l {
                l -= 1;
                self.push_left(l);
            }
            while nr < r {
                r -= 1;
                self.pop_right(r);
            }
            while l < nl {
                self.pop_left(l);
                l += 1;
            }
            self.assign(idx);
        }
    }
}

fn noshi91_order(query: &[(usize, usize)]) -> Vec<usize> {
    let q = query.len();
    let n = query.iter().map(|x| x.1).max().unwrap();
    let b = (n as f64 / (q as f64).sqrt()).max(1.) as usize;
    let mut ret1: Vec<_> = (0..q).collect();
    let mut ret2: Vec<_> = (0..q).collect();
    ret1.sort_by_cached_key(|&i| {
        let (l, r) = query[i];
        let x = l / b;
        let y = if x % 2 == 0 { r } else { n - r };
        (x, y)
    });
    ret2.sort_by_cached_key(|&i| {
        let (l, r) = query[i];
        let x = (l + b / 2) / b;
        let y = if x % 2 == 0 { r } else { n - r };
        (x, y)
    });
    if dist(query, &ret1) < dist(query, &ret2) {
        ret1
    } else {
        ret2
    }
}

fn dist(query: &[(usize, usize)], perm: &[usize]) -> usize {
    let (l, r) = query[perm[0]];
    perm.windows(2).fold(l + r, |acc, w| {
        let (l1, r1) = query[w[0]];
        let (l2, r2) = query[w[1]];
        acc + l1.abs_diff(l2) + r1.abs_diff(r2)
    })
}
