use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_what_was_written() {
    Command::cargo_bin("my-app")
        .unwrap()
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::diff("you wrote: hello\n"));
}

#[test]
fn fails_with_no_argument() {
    Command::cargo_bin("my-app")
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn fails_with_more_than_one_argument() {
    Command::cargo_bin("my-app")
        .unwrap()
        .arg("hello")
        .arg("world")
        .assert()
        .failure();
}
