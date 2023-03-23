//! 配列上の二分探索をします

pub trait BiSearchBy {
    type T;
    fn bisearch_by<F>(&self, f: F) -> usize
        where F: Fn(&Self::T) -> bool;
}

pub trait LowerBound {
    type T: PartialOrd;
    fn lower_bound(&self, v: Self::T) -> usize;
}

pub trait UpperBound {
    type T: PartialOrd;
    fn upper_bound(&self, v: Self::T) -> usize;
}

impl<T> BiSearchBy for [T] {
    type T = T;
    fn bisearch_by<F>(&self, f: F) -> usize
        where F: Fn(&Self::T) -> bool {
            let (mut ok, mut ng) = (self.len() as i32, -1);
            while ok.abs_diff(ng) > 1 {
                let mid = (ok + ng) / 2;
                if f(&self[mid as usize]) { ok = mid; } else { ng = mid; }
            }
            ok as _
        }
}

impl<T: PartialOrd> LowerBound for [T] {
    type T = T;
    fn lower_bound(&self, v: Self::T) -> usize {
        self.bisearch_by(|a| v <= *a)
    }
}

impl<T: PartialOrd> UpperBound for [T] {
    type T = T;
    fn upper_bound(&self, v: Self::T) -> usize {
        self.bisearch_by(|a| v < *a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v = vec![1, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9];
        assert_eq!(v.lower_bound(5), 6);
        assert_eq!(v.upper_bound(5), 7);
        assert_eq!(v.lower_bound(-1), 0);
        assert_eq!(v.upper_bound(-1), 0);
        assert_eq!(v.lower_bound(10), 13);
        assert_eq!(v.upper_bound(10), 13);
    }
}
