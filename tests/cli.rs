use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn fail_on_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo-rust")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}


#[test]
fn success() -> TestResult {
    let mut cmd = Command::cargo_bin("echo-rust")?;
    cmd.arg("sample").assert().success();
    Ok(())
}

fn run_test_file(args: &[&str], expected_file: &str) -> TestResult {
    let expected =  fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echo-rust")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn success_hello1() -> TestResult {
    run_test_file(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn success_hello2() -> TestResult {
    run_test_file(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn success_hello1_n() -> TestResult {
    run_test_file(&["-n", "Hello  there"], "tests/expected/hello1.n.txt")
}

#[test]
fn success_hello2_n() -> TestResult {
    run_test_file(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

