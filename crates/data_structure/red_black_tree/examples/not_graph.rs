use proconio::{input, marker::Usize1};
use qitoy_derive::query_readable;
use qitoy_red_black_tree::{MAct, RedBlackTree};

enum M {}
impl MAct for M {
    type S = i64;

    type F = i64;

    fn e() -> Self::S {
        0
    }

    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        a + b
    }

    fn id() -> Self::F {
        0
    }

    fn comp(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }

    fn map(f: &Self::F, x: &Self::S, len: usize) -> Self::S {
        f * len as i64 + x
    }
}

query_readable! {
    Query {
        {},
        { a: Usize1, b: usize, v: i64, },
        { a: Usize1, b: usize, c: Usize1, d: usize, },
        { a: Usize1, b: usize, },
    }
}

fn main() {
    input! {
        n: usize, q: usize,
        x: [i64; n],
        q: [Query; q],
    }
    let mut rbt: RedBlackTree<M> = x.into_iter().collect();
    for q in q {
        match q {
            Query::Query0 {} => todo!(),
            Query::Query1 { a, b, v } => rbt = rbt.apply(a..b, v),
            Query::Query2 { a, b, c, d } => {
                let (l, _, r) = rbt.split3(a..b);
                let (_, m, _) = rbt.split3(c..d);
                rbt = l.merge3(&m, &r);
            }
            Query::Query3 { a, b } => println!("{}", rbt.prod(a..b)),
        }
    }
}
