use super::Dfa;

pub struct And<D, E> {
    d: D,
    e: E,
}

impl<D, E> And<D, E> {
    pub(super) fn new(d: D, e: E) -> Self {
        Self { d, e }
    }
}

impl<D, E> Dfa for And<D, E>
where
    D: Dfa,
    E: Dfa<Alphabet = D::Alphabet>,
{
    type State = (D::State, E::State);
    type Alphabet = D::Alphabet;

    #[inline]
    fn trans(&self, state: &Self::State, alpha: &Self::Alphabet) -> Option<Self::State> {
        self.d
            .trans(&state.0, alpha)
            .zip(self.e.trans(&state.1, alpha))
    }

    #[inline]
    fn init(&self) -> Self::State {
        (self.d.init(), self.e.init())
    }

    #[inline]
    fn accept(&self, state: &Self::State) -> bool {
        self.d.accept(&state.0) && self.e.accept(&state.1)
    }
}
