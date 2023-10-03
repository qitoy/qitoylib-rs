use ac_library::Max;
use super::{verify, Verify};

struct RerootingDp;

impl Verify for RerootingDp {
    fn solve(input: &str, stdout: &mut String) {
        use proconio::{input, marker::Usize1, source::once::OnceSource};
        use qitoy_rerooting_dp::{rerooting_dp, Tree};

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
            .spawn(move || rerooting_dp::<Max<usize>>(&tree, &mut |v, _| v + 1, &mut |v, _| v))
            .unwrap()
            .join()
            .unwrap();

        for v in v {
            stdout.push_str(&format!("{}\n", 2 * n - 2 - v));
        }
    }
}

verify! {
    RerootingDp("aoj/1595")
}
