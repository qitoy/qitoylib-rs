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
    fn cmp(&self, rhs: &Self, range: &Range<i64>) -> Option<Ordering> {
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

pub struct LiChaoTree {
    /// (a, b): ax+b
    line: Line,
    /// (l, r): l..r
    range: Range<i64>,
    left: Option<NonNull<Self>>,
    right: Option<NonNull<Self>>,
}

impl LiChaoTree {
    pub fn new(range: Range<i64>) -> Self {
        Self {
            line: Line::new(),
            range,
            left: None,
            right: None,
        }
    }

    pub fn add_line(&mut self, a: i64, b: i64) {
        let line = Line { a, b };
        if self.len() == 1 {
            let start = self.range.start;
            if self.line.get(start) > line.get(start) {
                self.line = line;
            }
            return;
        }
        if let Some(ord) = self.line.cmp(&line, &self.range) {
            if ord == Greater {
                self.line = line;
            }
            return;
        }
        self.left_mut().add_line(a, b);
        self.right_mut().add_line(a, b);
    }

    pub fn get_min(&self, x: i64) -> i64 {
        let mut min = self.line.get(x);
        if let Some(left) = self.left() {
            min = min.min(left.get_min(x));
        }
        if let Some(right) = self.right() {
            min = min.min(right.get_min(x));
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
        let (left_range, _) = self.range_divide();
        unsafe {
            self.left
                .get_or_insert_with(|| NonNull::from(Box::leak(Box::new(Self::new(left_range)))))
                .as_mut()
        }
    }

    fn right_mut(&mut self) -> &mut Self {
        let (_, right_range) = self.range_divide();
        unsafe {
            self.right
                .get_or_insert_with(|| NonNull::from(Box::leak(Box::new(Self::new(right_range)))))
                .as_mut()
        }
    }

    fn len(&self) -> i64 {
        let Range { start, end } = self.range;
        end - start
    }

    fn mid(&self) -> i64 {
        let Range { start, end } = self.range;
        (start + end).div_euclid(2)
    }

    fn range_divide(&self) -> (Range<i64>, Range<i64>) {
        let Range { start, end } = self.range;
        let mid = self.mid();
        (start..mid, mid..end)
    }
}
