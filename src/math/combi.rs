use ac_library_rs::modint::ModIntBase;

pub struct Combi<T: ModIntBase> {
    fact: Vec<T>,
    factinv: Vec<T>,
}

impl<T: ModIntBase> Combi<T> {
    pub fn new(n: usize) -> Self {
        let mut fact = vec![T::new(1); n+1];
        for i in 1..=n {
            fact[i] = fact[i-1] * i.into();
        }
        let factinv = fact.iter().map(|v| v.inv()).collect();
        Self {
            fact,
            factinv,
        }
    }
    pub fn fact(&self, n: usize) -> T {
        self.fact[n]
    }
    pub fn factinv(&self, n: usize) -> T {
        self.factinv[n]
    }
    pub fn perm(&self, n: usize, k: usize) -> T {
        if n < k { T::raw(0) } else { self.fact[n] * self.factinv[n-k] }
    }
    pub fn combi(&self, n: usize, k: usize) -> T {
        if n < k { T::raw(0) } else { self.fact[n] * self.factinv[n-k] * self.factinv[k] }
    }
}
