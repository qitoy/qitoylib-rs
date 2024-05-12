// verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_path_composite_sum
use ac_library::{ModInt998244353 as Mint, Monoid};
use itertools::Itertools;
use proconio::input;
use qitoy_rerooting_dp::{rerooting_dp, Tree};

fn main() {
    input! {
        n: usize,
        a: [Mint; n],
        e: [(usize, usize, Mint, Mint); n-1],
    }
    let tree = e.iter().fold(Tree::new(n), |mut tree, &(u, v, _, _)| {
        tree.add_egde(u, v);
        tree
    });
    let res = rerooting_dp::<M>(
        &tree,
        &mut |(x, y), i| {
            let (_, _, b, c) = e[i];
            (b * x + c * y, y)
        },
        &mut |(x, y), i| (x + a[i], y + 1),
    );
    println!("{}", res.into_iter().map(|p| p.0).join(" "));
}

enum M {}
impl Monoid for M {
    type S = (Mint, Mint);

    fn identity() -> Self::S {
        (0.into(), 0.into())
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
}
