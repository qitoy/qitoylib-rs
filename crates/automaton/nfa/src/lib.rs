use qitoy_dfa::Dfa;
use std::collections::BTreeSet;

/// 非決定性有限オートマトン
pub trait Nfa {
    /// 状態の型
    type State: Ord + Clone;
    /// 文字の型
    type Alphabet;
    /// 遷移関数
    fn trans(&self, state: &Self::State, alpha: Option<&Self::Alphabet>) -> BTreeSet<Self::State>;
    /// 開始状態
    fn init(&self) -> BTreeSet<Self::State>;
    /// 受理関数
    fn accept(&self, state: &Self::State) -> bool;

    fn epsilon_closure(&self, mut state: BTreeSet<Self::State>) -> BTreeSet<Self::State> {
        let mut que = state.clone();
        while let Some(x) = que.pop_first() {
            state.insert(x.clone());
            self.trans(&x, None)
                .into_iter()
                .filter(|y| !state.contains(y))
                .for_each(|y| {
                    que.insert(y);
                });
        }
        state
    }

    #[inline]
    fn to_dfa(self) -> ToDfa<Self>
    where
        Self: Sized,
    {
        ToDfa { nfa: self }
    }
}

pub struct ToDfa<N> {
    nfa: N,
}

impl<N: Nfa> Dfa for ToDfa<N> {
    type State = BTreeSet<N::State>;
    type Alphabet = N::Alphabet;

    #[inline]
    fn trans(&self, state: &Self::State, alpha: &Self::Alphabet) -> Option<Self::State> {
        let ret = self.nfa.epsilon_closure(
            state
                .iter()
                .map(|s| self.nfa.trans(s, Some(alpha)))
                .fold(BTreeSet::new(), |a, x| &a | &x),
        );
        (!ret.is_empty()).then_some(ret)
    }

    #[inline]
    fn init(&self) -> Self::State {
        self.nfa.init()
    }

    #[inline]
    fn accept(&self, state: &Self::State) -> bool {
        state.iter().any(|s| self.nfa.accept(s))
    }
}
