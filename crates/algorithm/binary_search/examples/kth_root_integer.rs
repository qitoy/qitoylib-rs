// verification-helper: PROBLEM https://judge.yosupo.jp/problem/kth_root_integer
use proconio::input;
use qitoy_binary_search::binary_search;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            a: u64, k: u32,
        }
        // trivial
        if k == 1 {
            println!("{a}");
            continue;
        }
        println!(
            "{}",
            binary_search(0, u64::MAX, |mid| mid
                .checked_pow(k)
                .is_some_and(|p| p <= a))
        );
    }
}
