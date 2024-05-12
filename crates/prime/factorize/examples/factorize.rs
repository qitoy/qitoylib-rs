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
                std::iter::once(fac.len() as u64)
                    .chain(fac.into_iter())
                    .join(" ")
            })
            .join("\n")
    );
}
