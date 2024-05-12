use super::{Dfa, IntoDfa};
use std::collections::BTreeSet;

pub trait Nfa {
    type State: Ord + Clone;
    type Alphabet;
    fn trans(&self, state: &Self::State, alpha: Option<&Self::Alphabet>) -> BTreeSet<Self::State>;
    fn init(&self) -> BTreeSet<Self::State>;
    fn accept(&self, state: &Self::State) -> bool;

    #[inline]
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
}

pub struct NfaToDfa<N> {
    nfa: N,
}

impl<N: Nfa> Dfa for NfaToDfa<N> {
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
        Some(ret).filter(|r| !r.is_empty())
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

impl<N: Nfa> IntoDfa for N {
    type State = <NfaToDfa<N> as Dfa>::State;
    type Alphabet = <NfaToDfa<N> as Dfa>::Alphabet;
    type IntoDfa = NfaToDfa<N>;

    #[inline]
    fn into_dfa(self) -> Self::IntoDfa {
        NfaToDfa { nfa: self }
    }
}
