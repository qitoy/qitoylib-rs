use super::{verify, Verify};

struct Example;

impl Verify for Example {
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

verify! {
    Example("aoj/ITP1_1_D"),
}
