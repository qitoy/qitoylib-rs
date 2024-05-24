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
        if let Some(ord) = self.line.cmp(&line, &self.range) {
            if ord == Greater {
                self.line = line;
            }
            return;
        }
        let mid = self.mid();
        let left_range = Range {
            start: self.range.start,
            end: mid,
        };
        let right_range = Range {
            start: mid,
            end: self.range.end,
        };
        match (self.line.cmp(&line, &left_range), self.line.cmp(&line, &right_range)) {
            (None, None) => todo!(),
            (None, Some(_)) => todo!(),
            (Some(_), None) => todo!(),
            (Some(_), Some(_)) => todo!(),
        }
    }

    pub fn get_min(&self, x: i64) -> i64 {
        x
    }

    fn mid(&self) -> i64 {
        let Range { start, end } = self.range;
        (start + end).div_euclid(2)
    }
}
