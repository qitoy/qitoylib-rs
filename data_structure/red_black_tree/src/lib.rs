use std::ops::Range;
use std::rc::Rc;

pub trait MAct {
    /// element type
    type S: Clone;
    /// map type
    type F: Clone + PartialEq;
    /// identity element
    fn e() -> Self::S;
    /// binary operation
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
    /// identity map
    fn id() -> Self::F;
    /// composition
    /// `|x| f(g(x))`
    fn comp(f: &Self::F, g: &Self::F) -> Self::F;
    /// mapping
    /// `f(x)`
    /// * `len` - subtree's len (where subtree's value is x)
    fn map(f: &Self::F, x: &Self::S, len: usize) -> Self::S;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

enum Node<M: MAct> {
    Leaf {
        val: M::S,
    },
    Tree {
        left: Rc<Node<M>>,
        right: Rc<Node<M>>,
        color: Color,
        /// 葉までの黒ノードの数（自身は含まない）
        rank: usize,
        /// 部分木の葉ノード数
        len: usize,
        val: M::S,
        lazy: M::F,
        rev: bool,
    },
}

use Color::{Black, Red};
use Node::{Leaf, Tree};

impl<M: MAct> Clone for Node<M> {
    fn clone(&self) -> Self {
        match self {
            Leaf { val } => Leaf { val: val.clone() },
            Tree {
                left,
                right,
                color,
                rank,
                len,
                val,
                lazy,
                rev,
            } => Tree {
                left: left.clone(),
                right: right.clone(),
                color: *color,
                rank: *rank,
                len: *len,
                val: val.clone(),
                lazy: lazy.clone(),
                rev: *rev,
            },
        }
    }
}

impl<M: MAct> Default for Node<M> {
    fn default() -> Self {
        Leaf { val: M::e() }
    }
}

impl<M: MAct> Node<M> {
    fn new(left: &Rc<Self>, right: &Rc<Self>, color: Color) -> Rc<Self> {
        Tree {
            left: left.clone(),
            right: right.clone(),
            color,
            rank: left.rank()
                + match left.color() {
                    Black => 1,
                    Red => 0,
                },
            len: left.len() + right.len(),
            val: M::op(left.val(), right.val()),
            lazy: M::id(),
            rev: false,
        }
        .into()
    }

    fn leaf(val: M::S) -> Rc<Self> {
        Leaf { val }.into()
    }

    #[inline]
    fn left(&self) -> Rc<Self> {
        let Tree { left, .. } = self else { unreachable!(); };
        left.clone()
    }

    #[inline]
    fn right(&self) -> Rc<Self> {
        let Tree { right, .. } = self else { unreachable!(); };
        right.clone()
    }

    #[inline]
    fn color(&self) -> Color {
        match self {
            Leaf { .. } => Black,
            Tree { color, .. } => *color,
        }
    }

    #[inline]
    fn rank(&self) -> usize {
        match self {
            Leaf { .. } => 0,
            Tree { rank, .. } => *rank,
        }
    }

    #[allow(clippy::len_without_is_empty)]
    #[inline]
    fn len(&self) -> usize {
        match self {
            Leaf { .. } => 1,
            Tree { len, .. } => *len,
        }
    }

    #[inline]
    fn val(&self) -> &M::S {
        match self {
            Leaf { val } | Tree { val, .. } => val,
        }
    }

    fn push_new(self: &Rc<Self>) -> Rc<Self> {
        match self.as_ref() {
            Leaf { .. } => self.clone(),
            Tree {
                lazy, rev: false, ..
            } if lazy == &M::id() => self.clone(),
            Tree {
                left,
                right,
                color,
                lazy,
                rev,
                ..
            } => {
                let (left, right) = (left.update(lazy, *rev), right.update(lazy, *rev));
                if *rev {
                    Self::new(&right, &left, *color)
                } else {
                    Self::new(&left, &right, *color)
                }
            }
        }
    }

    fn lazy_push(&self) -> Option<(Rc<Self>, Rc<Self>)> {
        match self {
            Leaf { .. } => None,
            Tree {
                left,
                right,
                lazy,
                rev,
                ..
            } => {
                let (left, right) = (left.update(lazy, *rev), right.update(lazy, *rev));
                if *rev {
                    (right, left)
                } else {
                    (left, right)
                }
            }
            .into(),
        }
    }

    fn update(self: &Rc<Self>, lazy: &M::F, rev: bool) -> Rc<Self> {
        match (self.as_ref(), lazy, rev) {
            (Leaf { .. }, id, _) | (Tree { .. }, id, false) if id == &M::id() => self.clone(),
            _ => Node::clone(self).update_mut(lazy, rev),
        }
    }

    #[inline]
    fn update_mut(mut self, lazy: &M::F, rev: bool) -> Rc<Self> {
        match &mut self {
            Leaf { val: v } => {
                *v = M::map(lazy, v, 1);
            }
            Tree {
                val: v,
                lazy: l,
                rev: r,
                len,
                ..
            } => {
                *r ^= rev;
                *l = M::comp(lazy, l);
                *v = M::map(lazy, v, *len);
            }
        }
        self.into()
    }

    #[inline]
    fn to_black(self: &Rc<Self>) -> Rc<Self> {
        match self.color() {
            Red => {
                let (left, right) = self.lazy_push().unwrap();
                Self::new(&left, &right, Black)
            }
            Black => self.clone(),
        }
    }

    #[inline]
    fn merge(self: &Rc<Self>, v: &Rc<Self>) -> Rc<Self> {
        Self::merge_sub(self, v).to_black()
    }

    fn merge_sub(a: &Rc<Self>, b: &Rc<Self>) -> Rc<Self> {
        use std::cmp::Ordering::*;
        let (a, b) = (a.push_new(), b.push_new());
        match Ord::cmp(&a.rank(), &b.rank()) {
            Less => {
                let c = Self::merge_sub(&a, &b.left());
                if b.color() == Black && c.color() == Red && c.left().color() == Red {
                    if b.right().color() == Black {
                        Self::new(&c.left(), &Self::new(&c.right(), &b.right(), Red), Black)
                    } else {
                        Self::new(&c.to_black(), &b.right().to_black(), Red)
                    }
                } else {
                    Self::new(&c, &b.right(), b.color())
                }
            }
            Greater => {
                let c = Self::merge_sub(&a.right(), &b);
                if a.color() == Black && c.color() == Red && c.right().color() == Red {
                    if a.left().color() == Black {
                        Self::new(&Self::new(&a.left(), &c.left(), Red), &c.right(), Black)
                    } else {
                        Self::new(&a.left().to_black(), &c.to_black(), Red)
                    }
                } else {
                    Self::new(&a.left(), &c, a.color())
                }
            }
            Equal => Self::new(&a, &b, Red),
        }
    }

    fn split(self: &Rc<Self>, k: usize) -> (Rc<Self>, Rc<Self>) {
        use std::cmp::Ordering::*;
        let (left, right) = self.lazy_push().unwrap();
        match k.cmp(&left.len()) {
            Less => {
                let (l, r) = left.split(k);
                (l, r.merge(&right.to_black()))
            }
            Greater => {
                let (l, r) = right.split(k - left.len());
                (left.to_black().merge(&l), r)
            }
            Equal => (left.to_black(), right.to_black()),
        }
    }

    fn dump(self: &Rc<Self>, dump: &mut Vec<M::S>) {
        match self.lazy_push() {
            None => dump.push(self.val().clone()),
            Some((left, right)) => {
                left.dump(dump);
                right.dump(dump);
            }
        }
    }
}

pub struct RedBlackTree<M: MAct> {
    top: Option<Rc<Node<M>>>,
}

impl<M: MAct> Clone for RedBlackTree<M> {
    fn clone(&self) -> Self {
        Self {
            top: self.top.clone(),
        }
    }
}

impl<M: MAct> Default for RedBlackTree<M> {
    fn default() -> Self {
        Self { top: None }
    }
}

impl<M, T> From<T> for RedBlackTree<M>
where
    M: MAct,
    T: Into<Option<Rc<Node<M>>>>,
{
    fn from(value: T) -> Self {
        Self { top: value.into() }
    }
}

impl<M: MAct> RedBlackTree<M> {
    pub fn new(val: M::S) -> Self {
        Node::<M>::leaf(val).into()
    }

    pub fn len(&self) -> usize {
        self.top.as_ref().map_or(0, |top| top.len())
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    pub fn merge(&self, rhs: &Self) -> Self {
        match (&self.top, &rhs.top) {
            (None, b) => b.clone().into(),
            (a, None) => a.clone().into(),
            (Some(a), Some(b)) => a.merge(b).into(),
        }
    }

    pub fn split(&self, p: usize) -> (Self, Self) {
        assert!(p <= self.len());
        if p == 0 {
            return (Self::default(), self.clone());
        }
        if p == self.len() {
            return (self.clone(), Self::default());
        }
        let (l, r) = self.top.as_ref().unwrap().split(p);
        (l.into(), r.into())
    }

    pub fn split3(&self, range: Range<usize>) -> (Self, Self, Self) {
        let (l, r) = (range.start, range.end);
        let (m, r) = self.split(r);
        let (l, m) = m.split(l);
        (l, m, r)
    }

    pub fn insert(&self, p: usize, val: M::S) -> Self {
        let (l, r) = self.split(p);
        let m = Self::new(val);
        l.merge(&m).merge(&r)
    }

    pub fn erase(&self, p: usize) -> Self {
        let (l, _, r) = self.split3(p..p + 1);
        l.merge(&r)
    }

    pub fn prod(&self, range: Range<usize>) -> M::S {
        let (_, m, _) = self.split3(range);
        m.top.as_ref().map_or(M::e(), |top| top.val().clone())
    }

    pub fn apply(&self, range: Range<usize>, f: M::F) -> Self {
        if f == M::id() {
            return self.clone();
        }
        let (l, m, r) = self.split3(range);
        let m = m.top.map(|top| top.update(&f, false)).into();
        l.merge(&m).merge(&r)
    }

    pub fn reverse(&self, range: Range<usize>) -> Self {
        let (l, m, r) = self.split3(range);
        let m = m.top.map(|top| top.update(&M::id(), true)).into();
        l.merge(&m).merge(&r)
    }

    pub fn dump(&self) -> Vec<M::S> {
        let mut dump = Vec::with_capacity(self.len());
        if let Some(top) = &self.top {
            top.dump(&mut dump);
        }
        dump
    }
}

impl<M: MAct> FromIterator<M::S> for RedBlackTree<M> {
    fn from_iter<T: IntoIterator<Item = M::S>>(iter: T) -> Self {
        let data: Vec<_> = iter.into_iter().collect();
        merge_rec(&data)
    }
}

fn merge_rec<M: MAct>(data: &[M::S]) -> RedBlackTree<M> {
    let n = data.len();
    if n == 1 {
        RedBlackTree::new(data[0].clone())
    } else {
        merge_rec(&data[..n / 2]).merge(&merge_rec(&data[n / 2..]))
    }
}

#[cfg(test)]
mod test {
    use super::{MAct, RedBlackTree};

    struct MinAdd;
    /// min - add
    impl MAct for MinAdd {
        type S = i32;
        type F = i32;
        fn e() -> Self::S {
            i32::MAX
        }
        fn op(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
        fn id() -> Self::F {
            0
        }
        fn map(f: &Self::F, x: &Self::S, _: usize) -> Self::S {
            f + x
        }
        fn comp(f: &Self::F, g: &Self::F) -> Self::F {
            f + g
        }
    }

    #[test]
    fn test1() {
        // 1, 1, 4, 5, 1, 4
        let t1: RedBlackTree<MinAdd> = [1, 1, 4, 5, 1, 4].into_iter().collect();
        assert_eq!(t1.dump(), vec![1, 1, 4, 5, 1, 4]);
        // 1, 3, 6, 7, 1, 4
        let t2 = t1.apply(1..4, 2);
        assert_eq!(t2.dump(), vec![1, 3, 6, 7, 1, 4]);
        // [1, 1], [4, 5, 1], [4]
        let (l, _, r) = t1.split3(2..5);
        // [1, 3, 6], [7, 1, 4]
        let (_, m) = t2.split(3);
        // 1, 1, 7, 1, 4, 4
        let t3 = l.merge(&m).merge(&r);
        assert_eq!(t3.dump(), vec![1, 1, 7, 1, 4, 4]);
        // 1, 4, 1, 7, 1, 4
        let t4 = t3.reverse(1..5);
        assert_eq!(
            t4.dump(),
            vec![1, 4, 1, 7, 1, 4],
            "{:?}.reverse(1..5)",
            t3.dump()
        );
    }

    #[test]
    fn rev() {
        let n = 10;
        let t: RedBlackTree<MinAdd> = (0..n).collect();
        let t = t.reverse(0..n as usize);
        assert_eq!(t.dump(), (0..n).rev().collect::<Vec<_>>());
    }

    #[test]
    fn rev2() {
        let n = 5;
        let t1: RedBlackTree<MinAdd> = (0..n).collect();
        let t1 = t1.reverse(0..n as usize);
        let t2: RedBlackTree<MinAdd> = (0..n).map(|i| i + n).collect();
        let t2 = t2.reverse(0..n as usize);
        let t = t2.merge(&t1);
        let t = t.reverse(0..2 * n as usize);
        assert_eq!(t.dump(), (0..2 * n).collect::<Vec<_>>());
    }
}
