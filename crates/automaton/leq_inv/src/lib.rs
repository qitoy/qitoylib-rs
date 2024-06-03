use qitoy_dfa::Dfa;

/// `n`以下の非負整数を受理する（下の位から見る）
pub struct LeqInv<'a> {
    n: &'a [u8],
}

impl<'a> LeqInv<'a> {
    /// `n`を任意の進法で表記した配列から作成する
    pub fn new(n: &'a [u8]) -> Self {
        Self { n }
    }
}

impl Dfa for LeqInv<'_> {
    /// (index, greater)
    type State = (usize, bool);
    type Alphabet = u8;

    #[inline]
    fn trans(&self, state: &Self::State, alpha: &Self::Alphabet) -> Option<Self::State> {
        use std::cmp::Ordering::*;
        state.0.checked_sub(1)
            .map(|idx| {
                let d = self.n[idx];
                let greater = match alpha.cmp(&d) {
                    Less => false,
                    Equal => state.1,
                    Greater => true,
                };
                (idx, greater)
            })
    }

    #[inline]
    fn init(&self) -> Self::State {
        (self.n.len(), false)
    }

    #[inline]
    fn accept(&self, state: &Self::State) -> bool {
        state.0 == 0 && !state.1
    }
}
