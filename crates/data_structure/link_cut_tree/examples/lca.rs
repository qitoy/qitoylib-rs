// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca
use proconio::input;
use itertools::Itertools;
use qitoy_link_cut_tree::LinkCutTree;

fn main () {
    input! {
        n: usize, q: usize,
        p: [usize; n-1],
        q: [(usize, usize); q],
    }
    let mut lct = LinkCutTree::new(n);
    for (p, v) in p.into_iter().zip(1..) {
        lct.link(v, p);
    }
    //eprintln!("{lct:?}");
    println!("{}", q.into_iter().map(|(u, v)| lct.lca(u, v)).join("\n"));
}
