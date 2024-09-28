pub use qitoy_group::Group;
use std::convert::TryInto;

pub struct PotentializedUnionfind<T: Group> {
    /// if x is parent then -size, else parent
    parent_or_size: Vec<isize>,
    /// p = f(x)
    potential_diff: Vec<T::S>,
}

impl<T: Group> PotentializedUnionfind<T> {
    /// create new `unionfind with potential`
    pub fn new(n: usize) -> Self {
        Self {
            parent_or_size: vec![-1; n],
            potential_diff: vec![T::identity(); n],
        }
    }

    /// are `x` and `y` same group
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.leader_potential(x).0 == self.leader_potential(y).0
    }

    /// if `self.is_same(x, y)`, `Some(f)`, where `y = f(x)`
    /// else `None`
    pub fn diff(&mut self, x: usize, y: usize) -> Option<T::S> {
        if self.is_same(x, y) {
            // y = f(x), e = x(x), e = y(y) => f = y^{-1}x
            Some(T::binary_operation(
                &self.leader_potential(x).1,
                &T::inverse(&self.leader_potential(y).1),
            ))
        } else {
            None
        }
    }

    /// if `x` and `y` are same group, return false
    /// otherwise, merge as `y = f(x)`, and return true
    pub fn merge(&mut self, x: usize, y: usize, f: T::S) -> bool {
        // y = f(x), ye = h(y), xe = g(x) => ye = hfg^{-1}(xe)
        // (x, y, f) <- (xe, ye, hfg^{-1})
        let (x, g) = self.leader_potential(x);
        let (y, h) = self.leader_potential(y);
        if x == y {
            return false;
        }
        let f = T::binary_operation(&T::binary_operation(&T::inverse(&g), &f), &h);
        if self.parent_or_size[x] < self.parent_or_size[y] {
            return self.merge(y, x, T::inverse(&f));
        }
        // assert(size(x) <= size(y))
        // y = f(x)
        self.parent_or_size[y] += self.parent_or_size[x];
        self.parent_or_size[x] = y as isize;
        self.potential_diff[x] = f;
        true
    }

    /// `x`'s group's leader
    pub fn leader(&mut self, x: usize) -> usize {
        self.leader_potential(x).0
    }

    /// `(e, f)` where e is leader, x = f(e)
    fn leader_potential(&mut self, x: usize) -> (usize, T::S) {
        let Ok(p): Result<usize, _> = self.parent_or_size[x].try_into()
                   else { return (x, T::identity()); };
        // e = g(p), p = f(x) => e = gf(x)
        // where f = self.potential_diff[x]
        let (e, g) = self.leader_potential(p);
        self.potential_diff[x] = T::binary_operation(&self.potential_diff[x], &g);
        self.parent_or_size[x] = e as isize;
        (e, self.potential_diff[x].clone())
    }
}
