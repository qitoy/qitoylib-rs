use super::{AlphaTrans, And, Less, MultipleOf, NonZero};
use ac_library::Monoid;

/// 決定性有限オートマトン
pub trait Dfa {
    /// 状態の型
    type State: Ord;
    /// 文字の型
    type Alphabet;
    /// 遷移関数
    fn trans(&self, state: &Self::State, alpha: &Self::Alphabet) -> Option<Self::State>;
    /// 開始状態
    fn init(&self) -> Self::State;
    /// 受理関数
    fn accept(&self, state: &Self::State) -> bool;

    /// `Alphabet`が同じDFAの積をとる。
    #[inline]
    fn and<E>(self, other: E) -> And<Self, E>
    where
        Self: Sized,
        E: Dfa<Alphabet = Self::Alphabet>,
    {
        And::new(self, other)
    }

    /// `f`を通じて異なる`Alphabet`を受理できるようにする。
    #[inline]
    fn alpha_trans<F, A>(self, f: F) -> AlphaTrans<Self, F, A>
    where
        Self: Sized,
        F: Fn(&A) -> Self::Alphabet,
    {
        AlphaTrans::new(self, f)
    }

    /// `digit`進法で`m`の倍数になるとき受理する。
    #[inline]
    fn multiple_of(self, digit: u64, m: u64) -> And<Self, MultipleOf>
    where
        Self: Sized + Dfa<Alphabet = u8>,
    {
        self.and(MultipleOf::new(digit, m))
    }

    /// `n`をd進法表記の配列として、d進法で`n`以下の数字を受理する。
    #[inline]
    fn less(self, n: &[u8]) -> And<Self, Less>
    where
        Self: Sized + Dfa<Alphabet = u8>,
    {
        self.and(Less::new(n))
    }

    /// 非零の数字を受理する。
    #[inline]
    fn non_zero(self) -> And<Self, NonZero>
    where
        Self: Sized + Dfa<Alphabet = u8>,
    {
        self.and(NonZero::default())
    }

    /// DFAを用いたDPを計算する。
    #[inline]
    fn calc<M>(
        &self,
        alpha: impl IntoIterator<Item = Self::Alphabet> + Clone,
        len: usize,
        init: M::S,
        mut map: impl FnMut(&M::S, &Self::Alphabet) -> M::S,
    ) -> M::S
    where
        M: Monoid,
    {
        use std::collections::BTreeMap;

        let mut dp = BTreeMap::new();
        dp.insert(self.init(), init);

        for _ in 0..len {
            let mut ndp = BTreeMap::new();
            for (state, value) in dp {
                for a in alpha.clone() {
                    if let Some(nstate) = self.trans(&state, &a) {
                        let nvalue = map(&value, &a);
                        ndp.entry(nstate)
                            .and_modify(|v| *v = M::binary_operation(v, &nvalue))
                            .or_insert(nvalue);
                    }
                }
            }
            dp = ndp;
        }
        dp.into_iter()
            .filter_map(|v| if self.accept(&v.0) { Some(v.1) } else { None })
            .fold(M::identity(), |acc, x| M::binary_operation(&acc, &x))
    }
}
