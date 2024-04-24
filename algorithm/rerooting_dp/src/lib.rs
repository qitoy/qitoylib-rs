use ac_library::Monoid;
pub use qitoy_tree::{Edge, Tree};

pub fn rerooting_dp_subtree<T: Monoid>(
    tree: &Tree,
    add_edge: &mut impl FnMut(T::S, usize) -> T::S,
    add_vertex: &mut impl FnMut(T::S, usize) -> T::S,
) -> Vec<Vec<T::S>> {
    let mut dp = tree.iter().map(|v| vec![T::identity(); v.len()]).collect();
    dfs::<T>(tree, &mut dp, add_edge, add_vertex, 0, 0);
    bfs::<T>(tree, &mut dp, add_edge, add_vertex);
    dp
}

pub fn rerooting_dp<T: Monoid>(
    tree: &Tree,
    add_edge: &mut impl FnMut(T::S, usize) -> T::S,
    add_vertex: &mut impl FnMut(T::S, usize) -> T::S,
) -> Vec<T::S> {
    let dp = rerooting_dp_subtree::<T>(tree, add_edge, add_vertex);
    tree.iter()
        .enumerate()
        .map(|(v, tree)| {
            let ret = tree.iter().enumerate().fold(T::identity(), |acc, (i, e)| {
                T::binary_operation(&acc, &add_edge(dp[v][i].clone(), e.index))
            });
            add_vertex(ret, v)
        })
        .collect()
}

fn dfs<T: Monoid>(
    tree: &Tree,
    dp: &mut Vec<Vec<T::S>>,
    add_edge: &mut impl FnMut(T::S, usize) -> T::S,
    add_vertex: &mut impl FnMut(T::S, usize) -> T::S,
    v: usize,
    p: usize,
) -> T::S {
    let ret = tree[v].iter().enumerate().filter(|v| v.1.to != p).fold(
        T::identity(),
        |acc, (i, e)| {
            dp[v][i] = dfs::<T>(tree, dp, add_edge, add_vertex, e.to, v);
            T::binary_operation(&acc, &add_edge(dp[v][i].clone(), e.index))
        },
    );
    add_vertex(ret, v)
}

fn bfs<T: Monoid>(
    tree: &Tree,
    dp: &mut Vec<Vec<T::S>>,
    add_edge: &mut impl FnMut(T::S, usize) -> T::S,
    add_vertex: &mut impl FnMut(T::S, usize) -> T::S,
) {
    let mut que = std::collections::VecDeque::new();
    que.push_back((0, T::identity()));
    let mut seen = vec![false; dp.len()];
    seen[0] = true;
    while let Some((v, value)) = que.pop_front() {
        if let Some((i, _)) = tree[v].iter().enumerate().find(|v| seen[v.1.to]) {
            dp[v][i] = value;
        }
        let len = tree[v].len();
        let (mut cuml, mut cumr) = (vec![T::identity(); len + 1], vec![T::identity(); len + 1]);
        for i in 0..len {
            cuml[i + 1] =
                T::binary_operation(&cuml[i], &add_edge(dp[v][i].clone(), tree[v][i].index));
            let i = len - 1 - i;
            cumr[i] =
                T::binary_operation(&cumr[i + 1], &add_edge(dp[v][i].clone(), tree[v][i].index));
        }
        for i in 0..len {
            let u = tree[v][i].to;
            if !seen[u] {
                seen[u] = true;
                let value = add_vertex(T::binary_operation(&cuml[i], &cumr[i + 1]), v);
                que.push_back((u, value));
            }
        }
    }
}
