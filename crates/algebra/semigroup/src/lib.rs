use ac_library::Monoid;
use std::{convert::Infallible, marker::PhantomData};

pub trait SemiGroup {
    type S: Clone;
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
}

pub struct Maybe<T>(Infallible, PhantomData<fn() -> T>);

impl<T: SemiGroup> Monoid for Maybe<T> {
    type S = Option<T::S>;
    fn identity() -> Self::S {
        None
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        match (a, b) {
            (None, b) => b.clone(),
            (a, None) => a.clone(),
            (Some(a), Some(b)) => Some(T::binary_operation(a, b)),
        }
    }
}
