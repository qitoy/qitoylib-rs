use super::Dfa;

pub trait IntoDfa {
    type State;
    type Alphabet;
    type IntoDfa: Dfa<State = Self::State, Alphabet = Self::Alphabet>;
    fn into_dfa(self) -> Self::IntoDfa;
}
