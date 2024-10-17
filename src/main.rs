use std::io;

fn main() {
    println!("? Select the release type of change that you're committing: (Type number)");
    println!("  1. major");
    println!("  2. minor");
    println!("  3. patch");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if let Ok(choice) = input.trim().parse::<usize>() {
        match choice {
            1 => println!("You chose: major"),
            2 => println!("You chose: minor"),
            3 => println!("You chose: patch"),
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_input_1() -> TestResult {
        let mut cmd = Command::cargo_bin("wave-git-cz")?;
        cmd.write_stdin("1\n");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("You chose: major"))
            .stdout(predicate::str::contains("You chose: minor").not())
            .stdout(predicate::str::contains("You chose: patch").not())
            .stdout(predicate::str::contains("Invalid choice. Please try again.").not());

        Ok(())
    }

    #[test]
    fn test_input_2() -> TestResult {
        let mut cmd = Command::cargo_bin("wave-git-cz")?;
        cmd.write_stdin("2\n");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("You chose: major").not())
            .stdout(predicate::str::contains("You chose: minor"))
            .stdout(predicate::str::contains("You chose: patch").not())
            .stdout(predicate::str::contains("Invalid choice. Please try again.").not());

        Ok(())
    }
    #[test]
    fn test_input_3() -> TestResult {
        let mut cmd = Command::cargo_bin("wave-git-cz")?;
        cmd.write_stdin("3\n");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("You chose: major").not())
            .stdout(predicate::str::contains("You chose: minor").not())
            .stdout(predicate::str::contains("You chose: patch"))
            .stdout(predicate::str::contains("Invalid choice. Please try again.").not());

        Ok(())
    }

    #[test]
    fn test_input_4() -> TestResult {
        let mut cmd = Command::cargo_bin("wave-git-cz")?;
        cmd.write_stdin("4\n");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("You chose: major").not())
            .stdout(predicate::str::contains("You chose: minor").not())
            .stdout(predicate::str::contains("You chose: patch").not())
            .stdout(predicate::str::contains(
                "Invalid choice. Please try again.",
            ));

        Ok(())
    }

    #[test]
    fn test_input_a() -> TestResult {
        let mut cmd = Command::cargo_bin("wave-git-cz")?;
        cmd.write_stdin("a\n");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("You chose: major").not())
            .stdout(predicate::str::contains("You chose: minor").not())
            .stdout(predicate::str::contains("You chose: patch").not())
            .stdout(predicate::str::contains("Invalid choice. Please try again.").not());

        Ok(())
    }
}
