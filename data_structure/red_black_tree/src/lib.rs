use ac_library::{MapMonoid, Monoid};
use std::ops::Range;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

enum Node<M: MapMonoid> {
    Leaf {
        val: <M::M as Monoid>::S,
    },
    Tree {
        left: Rc<Node<M>>,
        right: Rc<Node<M>>,
        color: Color,
        /// 葉までの黒ノードの数（自身は含まない）
        rank: usize,
        /// 部分木の葉ノード数
        len: usize,
        val: <M::M as Monoid>::S,
        lazy: M::F,
        rev: bool,
    },
}

impl<M: MapMonoid> Clone for Node<M> {
    fn clone(&self) -> Self {
        match self {
            Self::Leaf { val } => Self::Leaf { val: val.clone() },
            Self::Tree {
                left,
                right,
                color,
                rank,
                len,
                val,
                lazy,
                rev,
            } => Self::Tree {
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

impl<M: MapMonoid> Node<M> {
    fn new(left: &Rc<Self>, right: &Rc<Self>, color: Color) -> Rc<Self> {
        Self::Tree {
            left: left.clone(),
            right: right.clone(),
            color,
            rank: left.rank()
                + match left.color() {
                    Color::Black => 1,
                    Color::Red => 0,
                },
            len: left.len() + right.len(),
            val: M::binary_operation(left.val(), right.val()),
            lazy: M::identity_map(),
            rev: false,
        }
        .into()
    }

    fn leaf(val: <M::M as Monoid>::S) -> Rc<Self> {
        Self::Leaf { val }.into()
    }

    fn left(&self) -> Rc<Self> {
        let Node::Tree { left, .. } = self else { unreachable!(); };
        left.clone()
    }

    fn right(&self) -> Rc<Self> {
        let Node::Tree { right, .. } = self else { unreachable!(); };
        right.clone()
    }

    fn color(&self) -> Color {
        match self {
            Self::Leaf { .. } => Color::Black,
            Self::Tree { color, .. } => *color,
        }
    }

    fn rank(&self) -> usize {
        match self {
            Self::Leaf { .. } => 0,
            Self::Tree { rank, .. } => *rank,
        }
    }

    #[allow(clippy::len_without_is_empty)]
    fn len(&self) -> usize {
        match self {
            Self::Leaf { .. } => 1,
            Self::Tree { len, .. } => *len,
        }
    }

    fn val(&self) -> &<M::M as Monoid>::S {
        match self {
            Self::Leaf { val } | Self::Tree { val, .. } => val,
        }
    }

    fn lazy_push(self: &Rc<Self>) -> Rc<Self> {
        match self.as_ref() {
            Self::Leaf { .. } => self.clone(),
            Self::Tree {
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

    fn update(&self, lazy: &M::F, rev: bool) -> Rc<Self> {
        let (l, r) = (lazy, rev);
        let mut node = self.clone();
        match &mut node {
            Self::Leaf { val } => {
                *val = M::mapping(l, val);
            }
            Self::Tree { val, lazy, rev, .. } => {
                *rev ^= r;
                *lazy = M::composition(l, lazy);
                *val = M::mapping(l, val);
            }
        }
        node.into()
    }

    fn to_black(self: &Rc<Self>) -> Rc<Self> {
        match self.color() {
            Color::Red => Self::new(&self.left(), &self.right(), Color::Black),
            Color::Black => self.clone(),
        }
    }

    fn merge(self: &Rc<Self>, v: &Rc<Self>) -> Rc<Self> {
        Self::merge_sub(self, v).to_black()
    }

    fn merge_sub(a: &Rc<Self>, b: &Rc<Self>) -> Rc<Self> {
        use std::cmp::Ordering::*;
        use Color::*;
        let (a, b) = (a.lazy_push(), b.lazy_push());
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
        let a = self.lazy_push();
        let Self::Tree { left, right, .. } = a.as_ref() else { unreachable!(); };
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
}

pub struct RedBlackTree<M: MapMonoid> {
    top: Option<Rc<Node<M>>>,
}

impl<M: MapMonoid> Clone for RedBlackTree<M> {
    fn clone(&self) -> Self {
        Self {
            top: self.top.clone(),
        }
    }
}

impl<M: MapMonoid> Default for RedBlackTree<M> {
    fn default() -> Self {
        Self { top: None }
    }
}

impl<M, T> From<T> for RedBlackTree<M>
where
    M: MapMonoid,
    T: Into<Option<Rc<Node<M>>>>,
{
    fn from(value: T) -> Self {
        Self { top: value.into() }
    }
}

impl<M: MapMonoid> RedBlackTree<M> {
    pub fn new(val: <M::M as Monoid>::S) -> Self {
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

    pub fn insert(&self, p: usize, val: <M::M as Monoid>::S) -> Self {
        let (l, r) = self.split(p);
        let m = Self::new(val);
        l.merge(&m).merge(&r)
    }

    pub fn erase(&self, p: usize) -> Self {
        let (l, _, r) = self.split3(p..p + 1);
        l.merge(&r)
    }

    pub fn prod(&self, range: Range<usize>) -> <M::M as Monoid>::S {
        let (_, m, _) = self.split3(range);
        m.top
            .as_ref()
            .map_or(M::identity_element(), |top| top.val().clone())
    }

    pub fn apply(&self, range: Range<usize>, f: M::F) -> Self {
        let (l, m, r) = self.split3(range);
        let m = m.top.map(|top| top.update(&f, false)).into();
        l.merge(&m).merge(&r)
    }

    pub fn reverse(&self, range: Range<usize>) -> Self {
        let (l, m, r) = self.split3(range);
        let m = m.top.map(|top| top.update(&M::identity_map(), true)).into();
        l.merge(&m).merge(&r)
    }
}
