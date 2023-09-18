extern crate ac_library;
use ac_library::Monoid;
use std::marker::PhantomData;

pub struct RerootingDP<T> {
    /// (index, vertex)
    tree: Vec<Vec<(usize, usize)>>,
    _cnt: usize,
    _phantom_data: PhantomData<fn() -> T>,
}

impl<T: Monoid> RerootingDP<T> {
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![vec![]; n],
            _cnt: 0,
            _phantom_data: PhantomData,
        }
    }

    pub fn add_edge(&mut self, v: usize, u: usize) {
        self.tree[v].push((self._cnt, u));
        self.tree[u].push((self._cnt, v));
        self._cnt += 1;
    }

    pub fn build_subtree<FV, FE>(&self, add_edge: &FE, add_vertex: &FV) -> Vec<Vec<T::S>>
    where
        FE: Fn(T::S, usize) -> T::S,
        FV: Fn(T::S, usize) -> T::S,
    {
        assert_eq!(self._cnt, self.tree.len() - 1);
        let mut dp = self
            .tree
            .iter()
            .map(|v| vec![T::identity(); v.len()])
            .collect();
        self.dfs(&mut dp, add_edge, add_vertex, 0, 0);
        self.bfs(&mut dp, add_edge, add_vertex);
        dp
    }

    pub fn build<FE, FV>(&self, add_edge: &FE, add_vertex: &FV) -> Vec<T::S>
    where
        FE: Fn(T::S, usize) -> T::S,
        FV: Fn(T::S, usize) -> T::S,
    {
        let dp = self.build_subtree(add_edge, add_vertex);
        self.tree
            .iter()
            .enumerate()
            .map(|(v, tree)| {
                tree.iter()
                    .enumerate()
                    .fold(T::identity(), |acc, (i, &(idx, _))| {
                        T::binary_operation(&acc, &add_edge(dp[v][i].clone(), idx))
                    })
            })
            .collect()
    }

    fn dfs<FE, FV>(
        &self,
        dp: &mut Vec<Vec<T::S>>,
        add_edge: &FE,
        add_vertex: &FV,
        v: usize,
        p: usize,
    ) -> T::S
    where
        FE: Fn(T::S, usize) -> T::S,
        FV: Fn(T::S, usize) -> T::S,
    {
        let ret = self.tree[v]
            .iter()
            .enumerate()
            .filter(|v| v.1 .1 != p)
            .fold(T::identity(), |acc, (i, &(idx, u))| {
                dp[v][i] = self.dfs(dp, add_edge, add_vertex, u, v);
                T::binary_operation(&acc, &add_edge(dp[v][i].clone(), idx))
            });
        add_vertex(ret, v)
    }

    fn bfs<FE, FV>(&self, dp: &mut Vec<Vec<T::S>>, add_edge: &FE, add_vertex: &FV)
    where
        FE: Fn(T::S, usize) -> T::S,
        FV: Fn(T::S, usize) -> T::S,
    {
        let mut que = std::collections::VecDeque::new();
        que.push_back((0, T::identity()));
        let mut seen = vec![false; dp.len()];
        seen[0] = true;
        while let Some((v, value)) = que.pop_front() {
            if let Some((i, _)) = self.tree[v].iter().enumerate().find(|v| seen[v.1 .1]) {
                dp[v][i] = value;
            }
            let len = self.tree[v].len();
            let (mut cuml, mut cumr) = (vec![T::identity(); len + 1], vec![T::identity(); len + 1]);
            for i in 0..len {
                cuml[i + 1] =
                    T::binary_operation(&cuml[i], &add_edge(dp[v][i].clone(), self.tree[v][i].0));
                let i = len - 1 - i;
                cumr[i] = T::binary_operation(
                    &cumr[i + 1],
                    &add_edge(dp[v][i].clone(), self.tree[v][i].0),
                );
            }
            for i in 0..len {
                let u = self.tree[v][i].1;
                if !seen[u] {
                    seen[u] = true;
                    let value = add_vertex(T::binary_operation(&cuml[i], &cumr[i + 1]), v);
                    que.push_back((u, value));
                }
            }
        }
    }
}
