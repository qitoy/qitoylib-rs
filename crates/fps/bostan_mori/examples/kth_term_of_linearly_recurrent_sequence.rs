// verification-helper: PROBLEM https://judge.yosupo.jp/problem/kth_term_of_linearly_recurrent_sequence
use ac_library::ModInt998244353 as Mint;
use proconio::input;
use qitoy_bostan_mori::bostan_mori;

fn main() {
    input! {
        d: usize, k: u64,
        a: [Mint; d],
        c: [Mint; d],
    }
    let c: Vec<_> = std::iter::once(1.into())
        .chain(c.into_iter().map(|c| -c))
        .collect();
    let mut b = ac_library::convolution(&a, &c);
    b.truncate(d);
    println!("{}", bostan_mori(&b, &c, k));
}
