use super::Dfa;

pub struct Less<'a> {
    n: &'a [u8],
}

impl<'a> Less<'a> {
    pub fn new(n: &'a [u8]) -> Self {
        Self { n }
    }
}

impl Dfa for Less<'_> {
    /// (index, smaller)
    type State = (usize, bool);
    type Alphabet = u8;

    #[inline]
    fn trans(&self, state: &Self::State, alpha: &Self::Alphabet) -> Option<Self::State> {
        use std::cmp::Ordering::*;
        self.n
            .get(state.0)
            .and_then(|d| match (state.1, alpha.cmp(d)) {
                (true, _) | (_, Less) => Some(true),
                (_, Equal) => Some(false),
                _ => None,
            })
            .map(|smaller| (state.0 + 1, smaller))
    }

    #[inline]
    fn init(&self) -> Self::State {
        (0, false)
    }

    #[inline]
    fn accept(&self, state: &Self::State) -> bool {
        state.0 == self.n.len()
    }
}

