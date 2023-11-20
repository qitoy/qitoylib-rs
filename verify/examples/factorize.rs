// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize
use itertools::Itertools as _;
use proconio::input;
use qitoy_prime_factorize::Factorize as _;

fn main() {
    input! {
        a: [u64],
    }
    println!(
        "{}",
        a.into_iter()
            .map(|a| {
                let fac = a.factorize().sorted().collect_vec();
                format!("{} {}", fac.len(), fac.iter().join(" "))
            })
            .join("\n")
    );
}
