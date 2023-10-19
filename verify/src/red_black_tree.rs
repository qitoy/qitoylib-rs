use super::{verify, Verify};

struct RedBlackTree;

impl Verify for RedBlackTree {
    fn solve(input: &str, stdout: &mut String) {
        use proconio::input;
        use qitoy_red_black_tree::{MAct, RedBlackTree};
        use std::fmt::Write;

        struct F;
        impl F {
            const MOD: u64 = 998244353;
        }
        impl MAct for F {
            type S = u64;
            type F = (u64, u64);
            fn e() -> Self::S {
                0
            }
            fn op(a: &Self::S, b: &Self::S) -> Self::S {
                let c = a + b;
                if c > Self::MOD {
                    c - Self::MOD
                } else {
                    c
                }
            }
            fn id() -> Self::F {
                (1, 0)
            }
            fn map(f: &Self::F, x: &Self::S, len: usize) -> Self::S {
                (f.0 * x + f.1 * len as u64) % Self::MOD
            }
            fn comp(f: &Self::F, g: &Self::F) -> Self::F {
                // f.0 * (g.0 * x + g.1) + f.1
                // = (f.0 * g.0) * x + f.0 * g.1 + f.1
                (f.0 * g.0 % Self::MOD, (f.0 * g.1 + f.1) % Self::MOD)
            }
        }

        let mut source = proconio::source::once::OnceSource::from(input);
        input! {
            from &mut source,
            n: usize, q: usize,
            a: [u64; n],
        }
        let mut rbt: RedBlackTree<F> = a.into_iter().collect();
        for _ in 0..q {
            input! {
                from &mut source,
                q: usize,
            }
            match q {
                0 => {
                    input! {
                        from &mut source,
                        i: usize, x: u64,
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
                        b: u64, c: u64,
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
    RedBlackTree("library_checker/dynamic_sequence_range_affine_range_sum"),
}
