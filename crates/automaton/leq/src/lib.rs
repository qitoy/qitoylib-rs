use qitoy_dfa::Dfa;

/// `n`以下の非負整数を受理する（上の位から見る）
pub struct Leq<'a> {
    n: &'a [u8],
}

impl<'a> Leq<'a> {
    /// `n`を任意の進法で表記した配列から作成する
    pub fn new(n: &'a [u8]) -> Self {
        Self { n }
    }
}

impl Dfa for Leq<'_> {
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

#[cfg(test)]
mod test {
    use super::Leq;
    use ac_library::Additive;
    use qitoy_dfa::Dfa;

    #[test]
    fn test() {
        let n = 1_000_000_007;
        let s: Vec<_> = n.to_string().bytes().map(|b| b - b'0').collect();
        let ans = Leq::new(&s).calc::<Additive<u32>>(0..10, s.len(), 1, |v, _| *v);
        assert_eq!(ans, n + 1);
    }

    #[test]
    fn test_bi() {
        let n = 998244353;
        let s: Vec<_> = format!("{n:b}").bytes().map(|b| b - b'0').collect();
        let ans = Leq::new(&s).calc::<Additive<u32>>(0..2, s.len(), 1, |v, _| *v);
        assert_eq!(ans, n + 1);
    }
}
