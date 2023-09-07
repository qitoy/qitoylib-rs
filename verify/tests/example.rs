use anyhow::{ensure, Result};
use rstest::rstest;
use std::fs;
use std::path::PathBuf;

fn solve(input: &str) -> String {
    use proconio::{input, source::once::OnceSource};
    let input = OnceSource::from(input);
    input! {
        from input,
        s: i32,
    }
    format!("{}:{}:{}\n", s / 3600, s / 60 % 60, s % 60)
}

fn check(_input: &str, output: &str, stdout: &str) -> bool {
    output == stdout
}

#[rstest]
fn example(#[files("testcases/aoj/ITP1_1_D/*")] path: PathBuf) -> Result<()> {
    let mut in_txt = path.clone();
    in_txt.push("in.txt");
    let mut out_txt = path.clone();
    out_txt.push("out.txt");
    let input = fs::read_to_string(in_txt)?;
    let output = fs::read_to_string(out_txt)?;
    let stdout = solve(&input);
    ensure!(
        check(&input, &output, &stdout),
        "input:\n{}\nexcept:\n{}\nbut actual:\n{}",
        input,
        output,
        stdout
    );
    Ok(())
}
