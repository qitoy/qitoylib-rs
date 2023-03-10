pub trait LowerBound {
    type T: Ord;
    fn lower_bound(&self, v: Self::T) -> usize;
}

pub trait UpperBound {
    type T: Ord;
    fn upper_bound(&self, v: Self::T) -> usize;
}

impl<T: Ord> LowerBound for [T] {
    type T = T;
    fn lower_bound(&self, v: Self::T) -> usize {
        let (mut ok, mut ng) = (self.len() as i32, -1);
        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;
            if v <= self[mid as usize] { ok = mid; } else { ng = mid; }
        }
        ok as _
    }
}

impl<T: Ord> UpperBound for [T] {
    type T = T;
    fn upper_bound(&self, v: Self::T) -> usize {
        let (mut ok, mut ng) = (self.len() as i32, -1);
        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;
            if v < self[mid as usize] { ok = mid; } else { ng = mid; }
        }
        ok as _
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
