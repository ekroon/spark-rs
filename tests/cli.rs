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

#[test]
fn use_ticks() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spark")?;

    cmd.arg("--ticks=▁▂▃▄▅▆▇█");
    for i in 1..=8 {
        cmd.arg(i.to_string());
    }
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("▁▂▃▄▅▆▇█"));

    Ok(())
}

#[test]
fn use_digit_ticks() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spark")?;

    cmd.arg("--ticks=12345678");
    for i in 1..=8 {
        cmd.arg(i.to_string());
    }
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("12345678"));

    Ok(())
}

#[test]
fn use_letter_ticks() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spark")?;

    cmd.arg("--ticks=1234");
    for i in 1..=8 {
        cmd.arg(i.to_string());
    }
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("11223344"));

    Ok(())
}
