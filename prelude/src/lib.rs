#[macro_export]
macro_rules! mvec {
    ($val:expr) => {
        $val
    };
    ($size:expr; $($rest:expr);+) => {
        vec![mvec!($($rest);+);$size]
    };
}

#[macro_export]
macro_rules! max {
    ($e:expr) => { $e };
    ($e:expr, $($rest:expr),*) => {
        $e.max(max!($($rest),*))
    }
}

#[macro_export]
macro_rules! chmax {
    ($a:expr, $($b:expr),*) => {
        {
            let tmp = $crate::max!($($b),*);
            if $a < tmp {
                $a = tmp;
                true
            } else { false }
        }
    }
}

#[macro_export]
macro_rules! min {
    ($e:expr) => { $e };
    ($e:expr, $($rest:expr),*) => {
        $e.min(min!($($rest),*))
    }
}

#[macro_export]
macro_rules! chmin {
    ($a:expr, $($b:expr),*) => {
        {
            let tmp = $crate::min!($($b),*);
            if $a > tmp {
                $a = tmp;
                true
            } else { false }
        }
    }
}

pub use qitoy_utils_bound::{LowerBound, UpperBound};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max() {
        assert_eq!(max!(3), 3);
        assert_eq!(max!(3, -3, 4), 4);
        assert_eq!(max!(2 + 1, 4), 4);
    }

    #[test]
    fn min() {
        assert_eq!(min!(3), 3);
        assert_eq!(min!(3, -3, 4), -3);
    }
}
