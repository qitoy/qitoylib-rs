// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_D
use proconio::input;

fn main() {
    input! {
        s: i32,
    }
    println!("{}:{}:{}", s / 3600, s / 60 % 60, s % 60);
}
