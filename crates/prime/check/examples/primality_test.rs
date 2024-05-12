// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primality_test
use itertools::Itertools;
use proconio::input;
use qitoy_prime_check::PrimeCheck as _;

fn main() {
    input! {
        n: [u64],
    }
    println!(
        "{}",
        n.into_iter()
            .map(|n| if n.prime_check() { "Yes" } else { "No" })
            .join("\n")
    );
}
