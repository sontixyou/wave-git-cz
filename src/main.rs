use clap::{Arg, ArgMatches, Command};

fn main() {
    let cli: ArgMatches = Command::new("wave-git-cz")
        .author("sontixyou")
        .version("0.0.1")
        .about("Semantic Git commits")
        .arg(
            Arg::new("commit_body")
                .value_name("TEXT")
                .help("Input commit body")
                .num_args(1)
                .required(true),
        )
        .get_matches();
    let text = cli.get_one::<String>("commit_body").unwrap();
    print!("{:?}", text);
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;
    #[test]
    fn test_main_no_commit_body() -> TestResult {
        let mut cmd = Command::cargo_bin("wave-git-cz")?;
        cmd.assert()
            .failure()
            .stdout("")
            .stderr("error: the following required arguments were not provided:\n  <TEXT>\n\nUsage: wave-git-cz <TEXT>\n\nFor more information, try '--help'.\n");
        Ok(())
    }

    #[test]
    fn test_main_with_commit_body() -> TestResult {
        let mut cmd = Command::cargo_bin("wave-git-cz")?;
        cmd.arg("commit_body")
            .assert()
            .success()
            .stdout(predicate::eq("\"commit_body\""))
            .stderr(predicate::eq(""));
        Ok(())
    }
}
