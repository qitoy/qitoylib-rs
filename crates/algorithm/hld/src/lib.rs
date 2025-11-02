pub use qitoy_tree::{Edge, Tree};

#[derive(Debug, Clone, Default)]
pub struct Node {
    /// 親のindex
    pub parent: Option<usize>,
    /// heavy childのindex
    pub heavy: Option<usize>,
    /// light childのindex
    pub light: Vec<usize>,
    /// `self`の属するheavy pathの深さ
    pub depth: usize,
    /// オイラーツアーで(in, out)
    pub euler: (usize, usize),
    /// `self`の属するheavy pathの代表元
    pub head: usize,
}

#[derive(Clone, Debug)]
pub enum HldInterval {
    HeavyPath(usize, usize),
    LightEdge(usize, usize),
}

#[derive(Clone, Debug)]
pub enum HldDirection {
    Ascend,
    Descend,
}

#[derive(Debug)]
pub struct Hld {
    data: Vec<Node>,
}

impl std::ops::Index<usize> for Hld {
    type Output = Node;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Hld {
    /// 引数の木について、頂点`r`を根としてHL分解する。
    pub fn new(tree: &Tree, root: usize) -> Self {
        let n = tree.len();
        let mut data = vec![Node::default(); n];
        Self::dfs_sz(tree, &mut data, &mut vec![0; n], root);
        Self::dfs_hld(tree, &mut data, &mut 0, root);
        Self { data }
    }

    pub fn set_query(&self, a: usize) -> usize {
        self[a].euler.0
    }

    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self[a].depth < self[b].depth {
            std::mem::swap(&mut a, &mut b);
        }
        while self[a].depth != self[b].depth {
            a = self[self[a].head].parent.unwrap();
        }
        while self[a].head != self[b].head {
            a = self[self[a].head].parent.unwrap();
            b = self[self[b].head].parent.unwrap();
        }
        if self[a].euler.0 < self[b].euler.0 {
            a
        } else {
            b
        }
    }

    pub fn path_query(
        &self,
        a: usize,
        b: usize,
    ) -> impl Iterator<Item = (HldInterval, HldDirection)> {
        let l = self.lca(a, b);
        let ascend = self.ascend(a, l);
        let descend = self.ascend(b, l);
        ascend.into_iter().chain(descend.into_iter().map(|(v, _)| {
            use HldInterval::{HeavyPath, LightEdge};
            let u = match v {
                HeavyPath(a, b) => HeavyPath(b, a),
                LightEdge(a, b) => LightEdge(b, a),
            };
            (u, HldDirection::Descend)
        }))
    }

    pub fn subtree_query(&self, a: usize) -> (usize, usize) {
        self[a].euler
    }

    fn ascend(&self, mut a: usize, b: usize) -> Vec<(HldInterval, HldDirection)> {
        use HldDirection::Ascend;
        use HldInterval::{HeavyPath, LightEdge};

        let mut v = vec![];
        while self[a].head != self[b].head {
            if a == self[a].head {
                let b = self[a].parent.unwrap();
                v.push((LightEdge(a, b), Ascend));
                a = b;
            } else {
                let b = self[a].head;
                v.push((HeavyPath(a, b), Ascend));
                a = b;
            }
        }
        if a != b {
            v.push((HeavyPath(a, b), Ascend));
        }
        v
    }

    fn dfs_sz(tree: &Tree, data: &mut [Node], size: &mut [usize], a: usize) {
        let p = data[a].parent;
        size[a] = tree[a]
            .iter()
            .filter(|e| p.is_none_or(|p| e.to != p))
            .fold(1, |acc, e| {
                data[e.to].parent = Some(a);
                Self::dfs_sz(tree, data, size, e.to);
                let light = if data[a].heavy.is_none_or(|u| size[u] < size[e.to]) {
                    data[a].heavy.replace(e.to)
                } else {
                    Some(e.to)
                };
                if let Some(u) = light {
                    data[a].light.push(u);
                }
                acc + size[e.to]
            });
    }

    fn dfs_hld(tree: &Tree, data: &mut [Node], cnt: &mut usize, a: usize) {
        data[a].euler.0 = *cnt;
        *cnt += 1;
        let Node { heavy, parent, .. } = data[a];
        if let Some(h) = heavy {
            data[h].head = data[a].head;
            data[h].depth = data[a].depth;
            Self::dfs_hld(tree, data, cnt, h);
        }
        // light
        tree[a]
            .iter()
            .filter(|e| parent.is_none_or(|p| e.to != p))
            .filter(|e| heavy.is_none_or(|h| e.to != h))
            .for_each(|e| {
                data[e.to].head = e.to;
                data[e.to].depth = data[a].depth + 1;
                Self::dfs_hld(tree, data, cnt, e.to);
            });
        data[a].euler.1 = *cnt;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn debug() {
        let tree: Tree = [0, 1, 1, 2, 2, 2, 6, 3].into_iter().zip(1..).collect();
        let hld = Hld::new(&tree, 0);
        dbg!(hld);
    }
}
