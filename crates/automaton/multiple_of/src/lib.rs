use qitoy_dfa::Dfa;

pub struct MultipleOf {
    d: u64,
    m: u64,
}

impl MultipleOf {
    pub fn new(digit: u64, modulus: u64) -> Self {
        Self { d: digit, m: modulus }
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
