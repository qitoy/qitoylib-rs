pub trait Merge {
    /// `self`と`rhs`がソート済みのとき、マージする
    fn merge(&self, rhs: &Self) -> Self;
}

impl<T: PartialOrd + Copy> Merge for Vec<T> {
    fn merge(&self, rhs: &Self) -> Self {
        let mut r = vec![];
        merge_inner(&mut r, self, rhs);
        r
    }
}

fn merge_inner<T: PartialOrd + Copy>(r: &mut Vec<T>, a: &[T], b: &[T]) {
    match (a, b) {
        ([], []) => (),
        ([], b) => r.extend_from_slice(b),
        (a, []) => r.extend_from_slice(a),
        (a, b) => {
            if a[0] > b[0] {
                r.push(b[0]);
                merge_inner(r, a, &b[1..]);
            } else {
                r.push(a[0]);
                merge_inner(r, &a[1..], b);
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = vec![1, 2, 3, 4];
        let b = vec![2, 4, 5, 6];
        let c = a.merge(&b);
        assert_eq!(c, vec![1, 2, 2, 3, 4, 4, 5, 6]);
    }
}
