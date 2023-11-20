// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1595
use ac_library::Max;
use proconio::{input, marker::Usize1};
use qitoy_rerooting_dp::{rerooting_dp, Tree};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    }
    let tree = Tree::from(&edges);
    let v = rerooting_dp::<Max<usize>>(&tree, &mut |v, _| v + 1, &mut |v, _| v);
    println!("{}", v.into_iter().map(|v| 2 * n - v - 2).join("\n"));
}
