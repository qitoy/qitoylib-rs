use std::cmp::Ordering::{self, *};
use std::ops::Range;
use std::ptr::NonNull;

#[derive(Clone, Copy, PartialEq, Eq)]
/// ax+b
struct Line {
    a: i64,
    b: i64,
}

impl Line {
    fn new() -> Self {
        Self { a: 0, b: i64::MAX }
    }

    /// get ax+b
    fn get(&self, x: i64) -> i64 {
        self.a * x + self.b
    }

    /// - forall x in range, self.get(x) < rhs.get(x) => Some(Less)
    /// - forall x in range, self.get(x) == rhs.get(x) => Some(Equal)
    /// - forall x in range, self.get(x) > rhs.get(x) => Some(Greater)
    /// - otherwise => None
    fn cmp(&self, rhs: &Self, range: Range<i64>) -> Option<Ordering> {
        if self == rhs {
            Some(Equal)
        } else if self.get(range.start) < rhs.get(range.start)
            && self.get(range.end) < rhs.get(range.end)
        {
            Some(Less)
        } else if self.get(range.start) > rhs.get(range.start)
            && self.get(range.end) > rhs.get(range.end)
        {
            Some(Greater)
        } else {
            None
        }
    }
}

struct Node {
    /// {a, b}: ax+b
    line: Line,
    left: Option<NonNull<Self>>,
    right: Option<NonNull<Self>>,
}

impl Node {
    fn new() -> Self {
        Self {
            line: Line::new(),
            left: None,
            right: None,
        }
    }

    fn add_line(&mut self, a: i64, b: i64, l: i64, r: i64) {
        let line = Line { a, b };
        if r - l == 1 {
            if self.line.get(l) > line.get(l) {
                self.line = line;
            }
            return;
        }
        if let Some(ord) = self.line.cmp(&line, l..r) {
            if ord == Greater {
                self.line = line;
            }
            return;
        }
        let m = (l + r) / 2;
        self.left_mut().add_line(a, b, l, m);
        self.right_mut().add_line(a, b, m, r);
    }

    fn add_segment(&mut self, range: Range<i64>, a: i64, b: i64, l: i64, r: i64) {
        if r <= range.start || range.end <= l {
            return;
        }
        if range.start <= l && r <= range.end {
            self.add_line(a, b, l, r);
            return;
        }
        let m = (l + r) / 2;
        self.left_mut().add_segment(range.clone(), a, b, l, m);
        self.right_mut().add_segment(range, a, b, m, r);
    }

    pub fn get_min(&self, x: i64, l: i64, r: i64) -> i64 {
        let mut min = self.line.get(x);
        let m = (l + r) / 2;
        if (l..m).contains(&x) {
            if let Some(left) = self.left() {
                min = min.min(left.get_min(x, l, m));
            }
        }
        if (m..r).contains(&x) {
            if let Some(right) = self.right() {
                min = min.min(right.get_min(x, m, r));
            }
        }
        min
    }

    fn left(&self) -> Option<&Self> {
        self.left.map(|ptr| unsafe { ptr.as_ref() })
    }

    fn right(&self) -> Option<&Self> {
        self.right.map(|ptr| unsafe { ptr.as_ref() })
    }

    fn left_mut(&mut self) -> &mut Self {
        unsafe {
            self.left
                .get_or_insert_with(|| NonNull::from(Box::leak(Box::new(Self::new()))))
                .as_mut()
        }
    }

    fn right_mut(&mut self) -> &mut Self {
        unsafe {
            self.right
                .get_or_insert_with(|| NonNull::from(Box::leak(Box::new(Self::new()))))
                .as_mut()
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        if let Some(left) = self.left {
            unsafe { drop(Box::from_raw(left.as_ptr())) }
        }
        if let Some(right) = self.right {
            unsafe { drop(Box::from_raw(right.as_ptr())) }
        }
    }
}

pub struct LiChaoTree {
    range: Range<i64>,
    top: Node,
}

impl LiChaoTree {
    pub fn new(range: Range<i64>) -> Self {
        Self {
            range,
            top: Node::new(),
        }
    }

    pub fn add_line(&mut self, a: i64, b: i64) {
        self.top.add_line(a, b, self.range.start, self.range.end);
    }

    pub fn add_segment(&mut self, range: Range<i64>, a: i64, b: i64) {
        self.top
            .add_segment(range, a, b, self.range.start, self.range.end);
    }

    pub fn get_min(&self, x: i64) -> i64 {
        self.top.get_min(x, self.range.start, self.range.end)
    }
}
