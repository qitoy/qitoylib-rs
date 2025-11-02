//! 配列上の二分探索をします

pub trait LowerBound {
    type T: PartialOrd;
    fn lower_bound(&self, v: Self::T) -> usize;
}

pub trait UpperBound {
    type T: PartialOrd;
    fn upper_bound(&self, v: Self::T) -> usize;
}

impl<T: PartialOrd> LowerBound for [T] {
    type T = T;
    fn lower_bound(&self, v: Self::T) -> usize {
        self.partition_point(|a| a < &v)
    }
}

impl<T: PartialOrd> UpperBound for [T] {
    type T = T;
    fn upper_bound(&self, v: Self::T) -> usize {
        self.partition_point(|a| a <= &v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v = [1, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9];
        assert_eq!(v.lower_bound(5), 6);
        assert_eq!(v.upper_bound(5), 7);
        assert_eq!(v.lower_bound(-1), 0);
        assert_eq!(v.upper_bound(-1), 0);
        assert_eq!(v.lower_bound(10), 13);
        assert_eq!(v.upper_bound(10), 13);
    }
}
