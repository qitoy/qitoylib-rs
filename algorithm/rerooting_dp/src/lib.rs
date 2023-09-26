extern crate ac_library;
use ac_library::Monoid;

pub struct RerootingDP {
    /// (index, vertex)
    tree: Vec<Vec<(usize, usize)>>,
    _cnt: usize,
}

impl RerootingDP {
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![vec![]; n],
            _cnt: 0,
        }
    }

    pub fn add_edge(&mut self, v: usize, u: usize) {
        self.tree[v].push((self._cnt, u));
        self.tree[u].push((self._cnt, v));
        self._cnt += 1;
    }

    pub fn build_subtree<T: Monoid>(
        &self,
        add_edge: &impl Fn(T::S, usize) -> T::S,
        add_vertex: &impl Fn(T::S, usize) -> T::S,
    ) -> Vec<Vec<T::S>> {
        assert_eq!(self._cnt, self.tree.len() - 1);
        let mut dp = self
            .tree
            .iter()
            .map(|v| vec![T::identity(); v.len()])
            .collect();
        self.dfs::<T>(&mut dp, add_edge, add_vertex, 0, 0);
        self.bfs::<T>(&mut dp, add_edge, add_vertex);
        dp
    }

    pub fn build<T: Monoid>(
        &self,
        add_edge: &impl Fn(T::S, usize) -> T::S,
        add_vertex: &impl Fn(T::S, usize) -> T::S,
    ) -> Vec<T::S> {
        let dp = self.build_subtree::<T>(add_edge, add_vertex);
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

    fn dfs<T: Monoid>(
        &self,
        dp: &mut Vec<Vec<T::S>>,
        add_edge: &impl Fn(T::S, usize) -> T::S,
        add_vertex: &impl Fn(T::S, usize) -> T::S,
        v: usize,
        p: usize,
    ) -> T::S {
        let ret = self.tree[v]
            .iter()
            .enumerate()
            .filter(|v| v.1 .1 != p)
            .fold(T::identity(), |acc, (i, &(idx, u))| {
                dp[v][i] = self.dfs::<T>(dp, add_edge, add_vertex, u, v);
                T::binary_operation(&acc, &add_edge(dp[v][i].clone(), idx))
            });
        add_vertex(ret, v)
    }

    fn bfs<T: Monoid>(
        &self,
        dp: &mut Vec<Vec<T::S>>,
        add_edge: &impl Fn(T::S, usize) -> T::S,
        add_vertex: &impl Fn(T::S, usize) -> T::S,
    ) {
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

impl From<&Vec<(usize, usize)>> for RerootingDP {
    fn from(value: &Vec<(usize, usize)>) -> Self {
        let mut tree = Self::new(value.len() + 1);
        for &(v, u) in value {
            tree.add_edge(v, u);
        }
        tree
    }
}
