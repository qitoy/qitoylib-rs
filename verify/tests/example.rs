use verify::{Verify, verify};

struct V;

#[verify("aoj/ITP1_1_D")]
impl Verify for V {
    fn solve(input: &str, stdout: &mut String) {
        use proconio::{input, source::once::OnceSource};
        use std::fmt::Write;
        let input = OnceSource::from(input);
        input! {
            from input,
            s: i32,
        }
        writeln!(stdout, "{}:{}:{}", s / 3600, s / 60 % 60, s % 60).unwrap();
    }
}
