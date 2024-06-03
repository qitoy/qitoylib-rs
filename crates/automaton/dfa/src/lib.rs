mod and;
mod alpha_trans;

use ac_library::Monoid;
use and::And;
use alpha_trans::AlphaTrans;

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

    /// DFAを用いたDPを計算する。
    /// * `M` - モノイド
    /// * `alpha` - 文字の集合
    /// * `len` - 受理する文字列の長さ
    /// * `init` - DPの初期値
    /// * `map` - DPの遷移関数
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

pub trait IntoDfa {
    type State;
    type Alphabet;
    type IntoDfa: Dfa<State = Self::State, Alphabet = Self::Alphabet>;
    fn into_dfa(self) -> Self::IntoDfa;
}
