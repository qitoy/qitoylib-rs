// verification-helper: PROBLEM https://judge.yosupo.jp/problem/find_linear_recurrence
use proconio::input;
use ac_library::ModInt998244353 as Mint;
use itertools::Itertools;
use qitoy_berlekamp_massey::berlekamp_massey;

fn main() {
    input! {
        a: [Mint],
    }
    let c = berlekamp_massey(&a);
    println!("{}\n{}", c.len()-1, c.iter().skip(1).map(|c| -c).join(" "));
}
