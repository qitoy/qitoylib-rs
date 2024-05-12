// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum
use proconio::input;
use qitoy_derive::query_readable;
use qitoy_red_black_tree::{MAct, RedBlackTree};

struct F;
impl F {
    const MOD: u64 = 998244353;
}
impl MAct for F {
    type S = u64;
    type F = (u64, u64);
    fn e() -> Self::S {
        0
    }
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        let c = a + b;
        if c > Self::MOD {
            c - Self::MOD
        } else {
            c
        }
    }
    fn id() -> Self::F {
        (1, 0)
    }
    fn map(f: &Self::F, x: &Self::S, len: usize) -> Self::S {
        (f.0 * x + f.1 * len as u64) % Self::MOD
    }
    fn comp(f: &Self::F, g: &Self::F) -> Self::F {
        // f.0 * (g.0 * x + g.1) + f.1
        // = (f.0 * g.0) * x + f.0 * g.1 + f.1
        (f.0 * g.0 % Self::MOD, (f.0 * g.1 + f.1) % Self::MOD)
    }
}

query_readable! {
    Query {
        { i: usize, x: u64 },
        { i: usize },
        { l: usize, r: usize },
        { l: usize, r: usize, b: u64, c: u64 },
        { l: usize, r: usize },
    }
}

fn main() {
    input! {
        n: usize, q: usize,
        a: [u64; n],
        q: [Query; q],
    }
    let mut rbt: RedBlackTree<F> = a.into_iter().collect();
    for q in q {
        match q {
            Query::Query0 { i, x } => {
                rbt = rbt.insert(i, x);
            }
            Query::Query1 { i } => {
                rbt = rbt.erase(i);
            }
            Query::Query2 { l, r } => {
                rbt = rbt.reverse(l..r);
            }
            Query::Query3 { l, r, b, c } => {
                rbt = rbt.apply(l..r, (b, c));
            }
            Query::Query4 { l, r } => {
                println!("{}", rbt.prod(l..r));
            }
        }
    }
}
