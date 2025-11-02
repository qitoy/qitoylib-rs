use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Edge {
    pub index: usize,
    pub from: usize,
    pub to: usize,
}

pub struct Tree {
    /// (index, to)
    data: Vec<Vec<Edge>>,
    _cnt: usize,
}

impl Tree {
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![vec![]; n],
            _cnt: 0,
        }
    }

    pub fn add_egde(&mut self, v: usize, u: usize) {
        self.data[v].push(Edge {
            index: self._cnt,
            from: v,
            to: u,
        });
        self.data[u].push(Edge {
            index: self._cnt,
            from: u,
            to: v,
        });
        self._cnt += 1;
    }

    pub fn is_valid(&self) -> bool {
        self._cnt + 1 == self.data.len()
    }
}

impl From<&Vec<(usize, usize)>> for Tree {
    fn from(value: &Vec<(usize, usize)>) -> Self {
        let mut tree = Self::new(value.len() + 1);
        for &(v, u) in value {
            tree.add_egde(v, u);
        }
        tree
    }
}

impl FromIterator<(usize, usize)> for Tree {
    fn from_iter<T: IntoIterator<Item = (usize, usize)>>(iter: T) -> Self {
        let edges = iter.into_iter().collect();
        Tree::from(&edges)
    }
}

impl Deref for Tree {
    type Target = Vec<Vec<Edge>>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Tree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
