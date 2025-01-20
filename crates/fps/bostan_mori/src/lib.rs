use ac_library::modint::{Modulus, StaticModInt};

pub fn bostan_mori<M: Modulus>(
    p: &[StaticModInt<M>],
    q: &[StaticModInt<M>],
    n: u64,
) -> StaticModInt<M> {
    if p.is_empty() {
        return 0.into();
    }

    let even = |p: Vec<_>| p.into_iter().step_by(2).collect();
    let odd = |p: Vec<_>| p.into_iter().skip(1).step_by(2).collect();

    let (mut p, mut q, mut n) = (p.to_owned(), q.to_owned(), n);
    while n > 0 {
        let qm: Vec<StaticModInt<M>> = q
            .iter()
            .zip([0, 1].into_iter().cycle())
            .map(|(&q, s)| if s == 0 { q } else { -q })
            .collect();
        let u = ac_library::convolution(&p, &qm);
        p = if n & 1 == 0 { even(u) } else { odd(u) };
        q = even(ac_library::convolution(&q, &qm));
        n >>= 1;
    }
    p[0] / q[0]
}
