use std::process::Command;
use assert_cmd::prelude::*;

use predicates::prelude::*; // Used for writing assertions

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>>{
    Command::cargo_bin("crabsay").expect("binary exists").assert().success().stdout(predicate::str::contains("..."));
    Ok(())
}