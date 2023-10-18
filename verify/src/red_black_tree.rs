use super::{verify, Verify};

struct RedBlackTree;

impl Verify for RedBlackTree {
    fn solve(input: &str, stdout: &mut String) {
        use ac_library::{MapMonoid, ModInt998244353, Monoid};
        use proconio::input;
        use qitoy_red_black_tree::RedBlackTree;
        use std::fmt::Write;

        struct M;
        impl Monoid for M {
            type S = ModInt998244353;
            fn identity() -> Self::S {
                0.into()
            }
            fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
                a + b
            }
        }

        struct F;
        impl MapMonoid for F {
            type M = M;
            type F = (ModInt998244353, ModInt998244353);
            fn identity_map() -> Self::F {
                (1.into(), 0.into())
            }
            fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
                f.0 * x + f.1
            }
            fn composition(f: &Self::F, g: &Self::F) -> Self::F {
                // f.0 * (g.0 * x + g.1) + f.1
                // = (f.0 * g.0) * x + f.0 * g.1 + f.1
                (f.0 * g.0, f.0 * g.1 + f.1)
            }
        }

        let mut source = proconio::source::once::OnceSource::from(input);
        input! {
            from &mut source,
            n: usize, q: usize,
            a: [ModInt998244353; n],
        }
        let mut rbt = RedBlackTree::<F>::default();
        for a in a {
            rbt = rbt.insert(rbt.len(), a);
        }
        for _ in 0..q {
            input! {
                from &mut source,
                q: usize,
            }
            match q {
                0 => {
                    input! {
                        from &mut source,
                        i: usize, x: ModInt998244353,
                    }
                    rbt = rbt.insert(i, x);
                }
                1 => {
                    input! {
                        from &mut source,
                        i: usize,
                    }
                    rbt = rbt.erase(i);
                }
                2 => {
                    input! {
                        from &mut source,
                        l: usize, r: usize,
                    }
                    rbt = rbt.reverse(l..r);
                }
                3 => {
                    input! {
                        from &mut source,
                        l: usize, r: usize,
                        b: ModInt998244353, c: ModInt998244353,
                    }
                    rbt = rbt.apply(l..r, (b, c));
                }
                4 => {
                    input! {
                        from &mut source,
                        l: usize, r: usize,
                    }
                    writeln!(stdout, "{}", rbt.prod(l..r)).unwrap();
                }
                _ => unreachable!(),
            }
        }
    }
}

verify! {
    // now bug!!
    // RedBlackTree("library_checker/dynamic_sequence_range_affine_range_sum"),
}
