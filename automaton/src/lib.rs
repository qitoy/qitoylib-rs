extern crate ac_library;

mod dfa;
pub use dfa::Dfa;

mod nfa;
pub use nfa::Nfa;

mod into_dfa;
pub use into_dfa::IntoDfa;

mod and;
pub use and::And;

mod alpha_trans;
pub use alpha_trans::AlphaTrans;

mod multiple_of;
pub use multiple_of::MultipleOf;

mod less;
pub use less::Less;

mod non_zero;
pub use non_zero::NonZero;

mod unit;
pub use unit::Unit;

pub fn dfa_new<T>() -> Unit<T> {
    Unit::<T>::default()
}

#[cfg(test)]
mod test {
    use super::*;
    use ac_library::{Additive, ModInt998244353 as Mint, Monoid};
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
            dfa_new()
                .multiple_of(2, 3)
                .less(&n)
                .non_zero()
                .alpha_trans(|v: &(_, _, _)| v.0)
                .and(
                    dfa_new()
                        .multiple_of(2, 8)
                        .less(&n)
                        .non_zero()
                        .alpha_trans(|v: &(_, _, _)| v.1)
                )
                .and(
                    dfa_new()
                        .multiple_of(2, 4)
                        .less(&n)
                        .non_zero()
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
