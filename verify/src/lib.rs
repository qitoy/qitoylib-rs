pub trait Verify {
    fn solve(input: &str, stdout: &mut String);
    fn check(input: &str, output: &str, stdout: &str) -> bool {
        let _ = input;
        output == stdout
    }
    fn verify(path: std::path::PathBuf) -> anyhow::Result<()> {
        let mut in_txt = path.clone();
        in_txt.push("in");
        let mut out_txt = path;
        out_txt.push("out");
        let input = std::fs::read_to_string(in_txt)?;
        let output = std::fs::read_to_string(out_txt)?;
        let mut stdout = String::new();
        Self::solve(&input, &mut stdout);
        anyhow::ensure!(
            Self::check(&input, &output, &stdout),
            "input:\n{}\nexcept:\n{}\nbut actual:\n{}",
            input,
            output,
            stdout
        );
        Ok(())
    }
}

pub use verify_proc::verify;

mod example;
mod mo_algorithm;
mod prime;
mod red_black_tree;
mod rerooting_dp;
mod weighted_dsu;
