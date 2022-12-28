use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn cli_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spark")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));

    Ok(())
}

#[test]
fn happy_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spark")?;

    for i in 1..=8 {
        cmd.arg(i.to_string());
    }
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("▁▂▃▄▅▆▇█"));

    Ok(())
}
