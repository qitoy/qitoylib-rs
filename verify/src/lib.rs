pub trait Verify {
    fn solve(input: &str, stdout: &mut String);
    fn check(_input: &str, output: &str, stdout: &str) -> bool {
        output == stdout
    }
    fn verify(path: std::path::PathBuf) -> anyhow::Result<()> {
        let mut in_txt = path.clone();
        in_txt.push("in.txt");
        let mut out_txt = path;
        out_txt.push("out.txt");
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
