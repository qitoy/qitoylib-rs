// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min
use proconio::input;
use qitoy_binary_trie::BinaryTrie;

fn main() {
    input! {
        q: usize,
        q: [(usize, u32); q],
    }
    let mut bt = BinaryTrie::<u32>::new();
    for (q, x) in q {
        match q {
            0 => {
                if bt.count(x) == 0 {
                    bt.insert(x);
                }
            }
            1 => bt.remove(x),
            2 => {
                bt.xor_all(x);
                println!("{}", bt.min().unwrap());
                bt.xor_all(x);
            }
            _ => unreachable!(),
        }
    }
}
