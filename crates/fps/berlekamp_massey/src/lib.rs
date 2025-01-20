pub fn berlekamp_massey<M: ac_library::modint::ModIntBase>(s: &[M]) -> Vec<M> {
    let n = s.len();
    let mut b: Vec<M> = Vec::with_capacity(n + 1);
    let mut c: Vec<M> = Vec::with_capacity(n + 1);
    b.push(1.into());
    c.push(1.into());
    let mut y = M::raw(1);
    for ed in 1..=n {
        let l = c.len();
        let mut x = M::default();
        for i in 0..l {
            x += c[i] * s[ed - l + i];
        }
        b.push(M::default());
        let m = b.len();
        if x == M::default() {
            continue;
        }
        let freq = x / y;
        if l < m {
            let tmp = c.clone();
            c.splice(0..0, vec![M::default(); m - l]);
            for i in 0..m {
                c[i] -= freq * b[i];
            }
            b = tmp;
            y = x;
        } else {
            for i in 0..m {
                c[l - m + i] -= freq * b[i];
            }
        }
    }
    c.reverse();
    c
}
