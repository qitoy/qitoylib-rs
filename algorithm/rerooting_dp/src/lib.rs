extern crate ac_library;
use ac_library::Monoid;
use std::marker::PhantomData;

pub struct RerootingDP<T, FV, FE> {
    /// (index, vertex)
    tree: Vec<Vec<(usize, usize)>>,
    add_vertex: FV,
    add_edge: FE,
    cnt: usize,
    _phantom_data: PhantomData<fn() -> T>,
}

impl<T, FV, FE> RerootingDP<T, FV, FE>
where T: Monoid,
      FV: FnMut(T::S, usize) -> T::S,
      FE: FnMut(T::S, usize) -> T::S,
{

    pub fn new(n: usize, add_vertex: FV, add_edge: FE) -> Self {
        Self {
            tree: vec![vec![]; n],
            add_vertex,
            add_edge,
            cnt: 0,
            _phantom_data: PhantomData::<fn() -> T>,
        }
    }

    pub fn add_edge(&mut self, v: usize, u: usize) {
        self.tree[v].push((self.cnt, u));
        self.tree[u].push((self.cnt, v));
        self.cnt += 1;
    }

    pub fn build(&mut self) -> Vec<T::S> {
        let n = self.tree.len();
        assert_eq!(self.cnt, n-1);
        let mut dp = Vec::with_capacity(n);
        for i in 0..n {
            dp.push(vec![T::identity(); self.tree[i].len()]);
        }
        self.dfs1(&mut dp, 0, 0);
        self.dfs2(&mut dp, 0, 0, T::identity());
        (0..n).map(|v| {
            let mut ret = T::identity();
            for i in 0..self.tree[v].len() {
                ret = T::binary_operation(&ret, &(self.add_edge)(dp[v][i].clone(), self.tree[v][i].0));
            }
            (self.add_vertex)(ret, v)
        }).collect()
    }

    fn dfs1(&mut self, dp: &mut Vec<Vec<T::S>>, v: usize, p: usize) -> T::S {
        let mut ret = T::identity();
        for i in 0..self.tree[v].len() {
            let (idx, u) = self.tree[v][i];
            if u == p { continue; }
            dp[v][i] = self.dfs1(dp, u, v);
            ret = T::binary_operation(&ret, &(self.add_edge)(dp[v][i].clone(), idx));
        }
        (self.add_vertex)(ret, v)
    }

    fn dfs2(&mut self, dp: &mut Vec<Vec<T::S>>, v: usize, p: usize, value: T::S) {
        let len = self.tree[v].len();
        let (mut cuml, mut cumr) = (vec![T::identity(); len+1], vec![T::identity(); len+1]);
        for i in 0..len {
            if self.tree[v][i].1 == p { dp[v][i] = value.clone(); }
        }
        for i in 0..len {
            cuml[i+1] = T::binary_operation(&cuml[i], &(self.add_edge)(dp[v][i].clone(), self.tree[v][i].0));
            let i = len-1-i;
            cumr[i] = T::binary_operation(&cumr[i+1], &(self.add_edge)(dp[v][i].clone(), self.tree[v][i].0));
        }
        for i in 0..len {
            let u = self.tree[v][i].1;
            if u != p {
                let value = (self.add_vertex)(T::binary_operation(&cuml[i], &cumr[i+1]), v);
                self.dfs2(dp, u, v, value);
            }
        }
    }

}
