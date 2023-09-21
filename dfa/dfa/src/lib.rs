extern crate ac_library;
use ac_library::Monoid;
use std::hash::Hash;
use std::marker::PhantomData;

/// 決定性有限オートマトン
pub trait Dfa {
    /// 状態の型
    type State: Eq + Hash;
    /// 文字の型
    type Alphabet;
    /// 遷移関数
    fn trans(&self, state: &Self::State, alpha: &Self::Alphabet) -> Option<Self::State>;
    /// 開始状態
    fn init(&self) -> Self::State;
    /// 受理関数
    fn accept(&self, state: &Self::State) -> bool;

    /// `Alphabet`が同じDFAの積をとる。
    #[inline]
    fn and<E>(self, other: E) -> And<Self, E::IntoDfa>
    where
        Self: Sized,
        E: IntoDfa<Alphabet = Self::Alphabet>,
    {
        And::new(self, other.into_dfa())
    }

    /// `f`を通じて異なる`Alphabet`を受理できるようにする。
    #[inline]
    fn alpha_trans<F, A>(self, f: F) -> AlphaTrans<Self, F, A>
    where
        Self: Sized,
        F: Fn(&A) -> Self::Alphabet,
    {
        AlphaTrans::new(self, f)
    }

    /// DFAを用いたDPを計算する。
    #[inline]
    fn calc<M>(
        &self,
        alpha: impl IntoIterator<Item = Self::Alphabet> + Clone,
        len: usize,
        init: M::S,
        mut map: impl FnMut(&M::S, &Self::Alphabet) -> M::S,
    ) -> M::S
    where
        M: Monoid,
    {
        use std::collections::HashMap;

        let mut dp = HashMap::new();
        let mut ndp = HashMap::new();

        dp.insert(self.init(), init);

        for _ in 0..len {
            for (state, value) in dp.drain() {
                for a in alpha.clone() {
                    if let Some(nstate) = self.trans(&state, &a) {
                        let nvalue = map(&value, &a);
                        ndp.entry(nstate)
                            .and_modify(|v| *v = M::binary_operation(v, &nvalue))
                            .or_insert(nvalue);
                    }
                }
            }
            std::mem::swap(&mut dp, &mut ndp);
        }
        dp.drain()
            .filter_map(|v| if self.accept(&v.0) { Some(v.1) } else { None })
            .fold(M::identity(), |acc, x| M::binary_operation(&acc, &x))
    }
}

pub trait IntoDfa {
    type State;
    type Alphabet;
    type IntoDfa: Dfa<State = Self::State, Alphabet = Self::Alphabet>;
    fn into_dfa(self) -> Self::IntoDfa;
}

impl<D: Dfa> IntoDfa for D {
    type State = D::State;
    type Alphabet = D::Alphabet;
    type IntoDfa = D;

    #[inline]
    fn into_dfa(self) -> Self::IntoDfa {
        self
    }
}

pub struct And<D, E> {
    d: D,
    e: E,
}

impl<D, E> And<D, E> {
    pub(self) fn new(d: D, e: E) -> Self {
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

pub struct AlphaTrans<D, F, A> {
    d: D,
    f: F,
    _phantomdata: PhantomData<fn() -> A>,
}

impl<D, F, A> AlphaTrans<D, F, A> {
    pub(self) fn new(d: D, f: F) -> Self {
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

pub struct MultipleOf {
    d: u64,
    m: u64,
}

impl MultipleOf {
    pub fn new(digit: u64, m: u64) -> Self {
        Self { d: digit, m }
    }
}

impl Dfa for MultipleOf {
    type State = u64;
    type Alphabet = u8;

    #[inline]
    fn trans(&self, state: &Self::State, alpha: &Self::Alphabet) -> Option<Self::State> {
        Some((self.d * *state + u64::from(*alpha)) % self.m)
    }

    #[inline]
    fn init(&self) -> Self::State {
        0
    }

    #[inline]
    fn accept(&self, state: &Self::State) -> bool {
        state == &0
    }
}

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

pub struct NonZero;

impl NonZero {
    pub fn new() -> Self {
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

#[cfg(test)]
mod test {
    use super::*;
    use ac_library::{Additive, ModInt998244353 as Mint};
    #[test]
    fn test1() {
        assert_eq!(
            MultipleOf::new(10, 3)
                .and(MultipleOf::new(10, 5))
                .and(Less::new(&[1, 0, 0]))
                .calc::<Additive<i32>>(0..=9, 3, 1, |x, _| *x),
            100 / 15 + 1
        );
    }

    struct M;

    impl Monoid for M {
        type S = Mint;
        fn identity() -> Self::S {
            Mint::raw(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }

    #[test]
    fn test2() {
        let n: Vec<_> = format!("{:#b}", 31415926535897932u64)[2..]
            .bytes()
            .map(|b| b - b'0')
            .collect();
        assert_eq!(
            MultipleOf::new(2, 3)
                .and(Less::new(&n))
                .and(NonZero::new())
                .alpha_trans(|v: &(_, _, _)| v.0)
                .and(
                    MultipleOf::new(2, 8)
                        .and(Less::new(&n))
                        .and(NonZero::new())
                        .alpha_trans(|v: &(_, _, _)| v.1)
                )
                .and(
                    MultipleOf::new(2, 4)
                        .and(Less::new(&n))
                        .and(NonZero::new())
                        .alpha_trans(|v: &(_, _, _)| v.2)
                )
                .calc::<M>(
                    [(0, 0, 0), (1, 1, 0), (1, 0, 1), (0, 1, 1)],
                    n.len(),
                    Mint::raw(1),
                    |x, _| *x
                )
                .val(),
            759934997
        )
    }
}
