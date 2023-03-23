#[macro_export]
macro_rules! chmax {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a = $b;
            true
        } else { false }
    }
}

#[macro_export]
macro_rules! chmin {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a = $b;
            true
        } else { false }
    }
}

pub extern crate qitoy_utils_bound;

pub use qitoy_utils_bound::{BiSearchBy, LowerBound, UpperBound};
