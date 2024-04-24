use ac_library::Monoid;
pub use qitoy_tree::{Edge, Tree};

pub fn rerooting_dp_subtree<T: Monoid>(
    tree: &Tree,
    add_edge: &mut impl FnMut(T::S, usize) -> T::S,
    add_vertex: &mut impl FnMut(T::S, usize) -> T::S,
) -> Vec<Vec<T::S>> {
    let mut dp = tree.iter().map(|v| vec![T::identity(); v.len()]).collect();
    bfs1::<T>(tree, &mut dp, add_edge, add_vertex);
    bfs2::<T>(tree, &mut dp, add_edge, add_vertex);
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

fn bfs1<T: Monoid>(
    tree: &Tree,
    dp: &mut Vec<Vec<T::S>>,
    add_edge: &mut impl FnMut(T::S, usize) -> T::S,
    add_vertex: &mut impl FnMut(T::S, usize) -> T::S,
) {
    let mut stk = Vec::with_capacity(dp.len());
    stk.push((0, 0, 0));
    for i in 0..dp.len() {
        let (v, p, _) = stk[i];
        for i in 0..tree[v].len() {
            let u = tree[v][i].to;
            if u != p {
                stk.push((u, v, i));
            }
        }
    }
    for (v, p, i) in stk.into_iter().rev() {
        let ret = tree[v]
            .iter()
            .enumerate()
            .filter(|(_, e)| e.to != p)
            .fold(T::identity(), |acc, (i, e)| {
                T::binary_operation(&acc, &add_edge(dp[v][i].clone(), e.index))
            });
        if v != p {
            dp[p][i] = add_vertex(ret, v);
        }
    }
}

fn bfs2<T: Monoid>(
    tree: &Tree,
    dp: &mut Vec<Vec<T::S>>,
    add_edge: &mut impl FnMut(T::S, usize) -> T::S,
    add_vertex: &mut impl FnMut(T::S, usize) -> T::S,
) {
    let mut que = Vec::with_capacity(dp.len());
    que.push((0, 0, T::identity()));
    while let Some((v, p, value)) = que.pop() {
        if let Some((i, _)) = tree[v].iter().enumerate().find(|(_, e)| e.to == p) {
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
            if u != p {
                let value = add_vertex(T::binary_operation(&cuml[i], &cumr[i + 1]), v);
                que.push((u, v, value));
            }
        }
    }
}
