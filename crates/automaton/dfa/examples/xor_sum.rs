use ac_library::Monoid;
use proconio::input;
use qitoy_dfa::Dfa;
use qitoy_dfa_leq_inv::LeqInv;
use qitoy_nfa::Nfa;

enum ModInt {}
impl ModInt {
    const MOD: u32 = 1_000_000_007;
}
impl Monoid for ModInt {
    type S = u32;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let c = a + b;
        if c >= Self::MOD {
            c - Self::MOD
        } else {
            c
        }
    }
}

struct XorSum;
impl Nfa for XorSum {
    /// 繰り上がり
    type State = u8;

    type Alphabet = (u8, u8);

    fn trans(
        &self,
        state: &Self::State,
        alpha: Option<&Self::Alphabet>,
    ) -> std::collections::BTreeSet<Self::State> {
        let Some(&(u, v)) = alpha else { return Default::default(); };
        let mut ret = std::collections::BTreeSet::new();
        for a in 0..2 {
            for b in 0..2 {
                if (a ^ b == u) && (a + b + state) % 2 == v {
                    ret.insert((a + b + state) / 2);
                }
            }
        }
        ret
    }

    fn init(&self) -> std::collections::BTreeSet<Self::State> {
        std::iter::once(0).collect()
    }

    fn accept(&self, state: &Self::State) -> bool {
        state == &0
    }
}

fn main() {
    input! { n: u64 }
    let n: Vec<_> = format!("{n:b}").bytes().map(|b| b - b'0').collect();
    let ans = LeqInv::new(&n)
        .alpha_trans(|a: &(u8, u8)| a.0)
        .and(LeqInv::new(&n).alpha_trans(|a: &(u8, u8)| a.1))
        .and(XorSum.to_dfa())
        .calc::<ModInt>([(0, 0), (0, 1), (1, 0), (1, 1)], n.len(), 1, |v, _| *v);
    println!("{ans}");
}
