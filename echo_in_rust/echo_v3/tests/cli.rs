use assert_cmd::Command;
use std::{fs};

use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]

// echo在没有参数的情况下会运行失败
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echo")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?; // 注意问号
    Command::cargo_bin("echo")? // 注意问号
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}
