use super::{Dfa, IntoDfa};
use std::collections::BTreeSet;

pub trait Nfa {
    type State: Eq + Ord;
    type Alphabet;
    fn trans(&self, state: &Self::State, alpha: Option<&Self::Alphabet>) -> BTreeSet<Self::State>;
    fn init(&self) -> BTreeSet<Self::State>;
    fn accept(&self, state: &Self::State) -> bool;
}

pub struct NfaToDfa<N> {
    nfa: N,
}

impl<N: Nfa> Dfa for NfaToDfa<N> {
    type State = BTreeSet<N::State>;
    type Alphabet = N::Alphabet;
    fn trans(&self, state: &Self::State, alpha: &Self::Alphabet) -> Option<Self::State> {
       todo!(); 
    }
    fn init(&self) -> Self::State {
       self.nfa.init() 
    }
    fn accept(&self, state: &Self::State) -> bool {
       state.iter().any(|s| self.nfa.accept(s)) 
    }
}

impl<N: Nfa> IntoDfa for N {
    type State = <NfaToDfa<N> as Dfa>::State;
    type Alphabet = <NfaToDfa<N> as Dfa>::Alphabet;
    type IntoDfa = NfaToDfa<N>;
    fn into_dfa(self) -> Self::IntoDfa {
       NfaToDfa { nfa: self } 
    }
}
