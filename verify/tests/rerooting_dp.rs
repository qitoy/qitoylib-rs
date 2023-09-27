use ac_library::Monoid;
use verify::{verify, Verify};

struct M;

impl Monoid for M {
    type S = (usize, usize);
    fn identity() -> Self::S {
        (0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1.max(b.1))
    }
}

struct V;

#[verify("aoj/1595")]
impl Verify for V {
    fn solve(input: &str, stdout: &mut String) {
        use proconio::{input, marker::Usize1, source::once::OnceSource};
        use qitoy_rerooting_dp::{Tree, rerooting_dp};

        let mut source = OnceSource::from(input);

        input! {
            from &mut source,
            n: usize,
            edges: [(Usize1, Usize1); n - 1],
        }

        let tree = Tree::from(&edges);

        // avoid stack overflow
        let v = std::thread::Builder::new()
            .stack_size(n * 2048)
            .spawn(move || rerooting_dp::<M>(&tree, &mut |v, _| (v.0 + 1, v.1 + 1), &mut |v, _| v))
            .unwrap()
            .join()
            .unwrap();

        for v in v {
            stdout.push_str(&format!("{}\n", 2 * v.0 - v.1));
        }
    }
}
