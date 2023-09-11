mod check {
    use verify::{verify, Verify};

    struct V;

    #[verify("library_checker/primality_test")]
    impl Verify for V {
        fn solve(input: &str, stdout: &mut String) {
            use proconio::input;
            use qitoy_prime_check::PrimeCheck as _;
            let mut source = proconio::source::once::OnceSource::from(input);
            input! {
                from &mut source,
                q: usize,
            }
            for _ in 0..q {
                input! {
                    from &mut source,
                    n: u64,
                }
                stdout.push_str(if n.prime_check() { "Yes\n" } else { "No\n" });
            }
        }
    }
}

mod factorize {
    use verify::{verify, Verify};

    struct V;

    #[verify("library_checker/factorize")]
    impl Verify for V {
        fn solve(input: &str, stdout: &mut String) {
            use itertools::Itertools as _;
            use proconio::input;
            use qitoy_prime_factorize::Factorize as _;
            use std::fmt::Write;
            let mut source = proconio::source::once::OnceSource::from(input);
            input! {
                from &mut source,
                a: [u64],
            }
            writeln!(
                stdout,
                "{}",
                a.into_iter()
                    .map(|a| {
                        let fac = a.factorize().sorted().collect_vec();
                        format!("{} {}", fac.len(), fac.iter().join(" "))
                    })
                    .join("\n")
            )
            .unwrap();
        }

        // for diff "\n"
        fn check(input: &str, output: &str, stdout: &str) -> bool {
            use proconio::input;
            input! {
                from proconio::source::once::OnceSource::from(input),
                q: usize,
            }
            input! {
                from proconio::source::once::OnceSource::from(output),
                output: [[u64]; q],
            }
            input! {
                from proconio::source::once::OnceSource::from(stdout),
                stdout: [[u64]; q],
            }
            output == stdout
        }
    }
}
