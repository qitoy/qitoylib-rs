pub fn binary_search<T: Int>(mut ok: T, mut ng: T, mut is_ok: impl FnMut(T) -> bool) -> T {
    while T::diffgt1(ok, ng) {
        let mid = T::midpoint(ok, ng);
        *(if is_ok(mid) { &mut ok } else { &mut ng }) = mid;
    }
    ok
}

pub trait Int: Copy {
    fn diffgt1(a: Self, b: Self) -> bool;
    fn midpoint(a: Self, b: Self) -> Self;
}

macro_rules! int_impl {
    ($ty:ty, $wide:ty) => {
        impl Int for $ty {
            fn diffgt1(a: Self, b: Self) -> bool {
                a.abs_diff(b) > 1
            }
            fn midpoint(a: Self, b: Self) -> Self {
                ((a as $wide + b as $wide) / 2) as $ty
            }
        }
    };
}

int_impl!(u32, u128);
int_impl!(u64, u128);
int_impl!(usize, u128);
int_impl!(i32, i128);
int_impl!(i64, i128);
int_impl!(isize, i128);
