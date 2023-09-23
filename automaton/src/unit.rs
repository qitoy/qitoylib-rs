use super::Dfa;
use std::marker::PhantomData;

pub struct Unit<A> {
    _p: PhantomData<fn() -> A>,
}

#[allow(clippy::new_without_default)]
impl<A> Unit<A> {
    pub fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl<A> Dfa for Unit<A> {
    type State = ();
    type Alphabet = A;

    #[inline]
    fn trans(&self, _state: &Self::State, _alpha: &Self::Alphabet) -> Option<Self::State> {
        Some(())
    }

    #[inline]
    fn init(&self) -> Self::State {}

    #[inline]
    fn accept(&self, _state: &Self::State) -> bool {
        true
    }
}
