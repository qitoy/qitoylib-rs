use super::Dfa;
use std::marker::PhantomData;

pub struct AlphaTrans<D, F, A> {
    d: D,
    f: F,
    _phantomdata: PhantomData<fn() -> A>,
}

impl<D, F, A> AlphaTrans<D, F, A> {
    pub(super) fn new(d: D, f: F) -> Self {
        Self {
            d,
            f,
            _phantomdata: PhantomData,
        }
    }
}

impl<D, F, A> Dfa for AlphaTrans<D, F, A>
where
    D: Dfa,
    F: Fn(&A) -> D::Alphabet,
{
    type State = D::State;
    type Alphabet = A;

    #[inline]
    fn trans(&self, state: &Self::State, alpha: &Self::Alphabet) -> Option<Self::State> {
        self.d.trans(state, &(self.f)(alpha))
    }

    #[inline]
    fn init(&self) -> Self::State {
        self.d.init()
    }

    #[inline]
    fn accept(&self, state: &Self::State) -> bool {
        self.d.accept(state)
    }
}
