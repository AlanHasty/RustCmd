use assert_cmd::Command
use predicates::prelude::*;


#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("ME").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}