use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

fn echor_run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("echor")?
        .args(args)
        .output()
        .expect("fail");

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn runs() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();

    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    echor_run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> Result<()> {
    echor_run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<()> {
    echor_run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> Result<()> {
    echor_run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
