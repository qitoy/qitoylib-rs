// verification-helper: PROBLEM https://judge.yosupo.jp/problem/counting_primes
use proconio::input;
use qitoy_prime_pi::pi;

fn main() {
    input! {
        n: usize,
    }
    println!("{}", pi(n));
}
