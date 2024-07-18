// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group
#![allow(clippy::needless_range_loop)]
use ac_library::{ModInt998244353 as Mint, Monoid};
use proconio::{input, source::Readable};
use qitoy_derive::query_readable;
use qitoy_group::Group;
use qitoy_potentialized_unionfind::PotentializedUnionfind;

query_readable! {
    Query {
        { u: usize, v: usize, x: G, },
        { u: usize, v: usize }
    }
}

enum G {}
impl Monoid for G {
    type S = [[Mint; 2]; 2];

    fn identity() -> Self::S {
        [[1.into(), 0.into()], [0.into(), 1.into()]]
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let mut c = [[Mint::default(); 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    c[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        c
    }
}
impl Group for G {
    fn inverse(a: &Self::S) -> Self::S {
        [[a[1][1], -a[0][1]], [-a[1][0], a[0][0]]]
    }
}
impl Readable for G {
    type Output = <G as Monoid>::S;

    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {
        let mut out = [[Mint::default(); 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                out[i][j] = Mint::read(source);
            }
        }
        out
    }
}

fn main() {
    input! {
        n: usize, q: [Query],
    }
    let mut puf = PotentializedUnionfind::<G>::new(n);
    for q in q {
        match q {
            Query::Query0 { u, v, x } => {
                if puf.diff(v, u).is_some_and(|y| x != y) {
                    println!("0");
                } else {
                    puf.merge(v, u, x);
                    println!("1");
                }
            }
            Query::Query1 { u, v } => {
                if let Some(x) = puf.diff(v, u) {
                    println!("{} {} {} {}", x[0][0], x[0][1], x[1][0], x[1][1]);
                } else {
                    println!("-1");
                }
            }
        }
    }
}
