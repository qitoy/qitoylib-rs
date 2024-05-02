// verification-helper: PROBLEM https://judge.yosupo.jp/problem/counting_primes
use qitoy_prime_pi::pi;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    println!("{}", pi(n));
}
