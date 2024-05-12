use super::Dfa;

pub struct NonZero;

impl Default for NonZero {
    fn default() -> Self {
        Self
    }
}

impl Dfa for NonZero {
    type State = bool;
    type Alphabet = u8;

    #[inline]
    fn trans(&self, state: &Self::State, alpha: &Self::Alphabet) -> Option<Self::State> {
        Some(*state || alpha != &0)
    }

    #[inline]
    fn init(&self) -> Self::State {
        false
    }

    #[inline]
    fn accept(&self, state: &Self::State) -> bool {
        *state
    }
}
