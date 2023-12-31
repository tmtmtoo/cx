use assert_cmd::Command;
use predicates::prelude::*;

#[cfg(not(target_os = "windows"))]
#[test]
fn successful_1_time() {
    let mut cmd = Command::cargo_bin("cx").unwrap();

    cmd.arg("retry")
        .arg("-c")
        .arg("2")
        .arg("--")
        .arg("echo abc")
        .assert()
        .success()
        .stdout(predicate::eq(
            r"abc
",
        ));
}

#[cfg(target_os = "windows")]
#[test]
fn successful_1_time() {
    let mut cmd = Command::cargo_bin("cx").unwrap();

    cmd.arg("retry")
        .arg("-c")
        .arg("2")
        .arg("--")
        .arg("echo abc")
        .assert()
        .success()
        .stdout(predicate::eq("abc\n"));
}

#[test]
fn failed_2_time() {
    let mut cmd = Command::cargo_bin("cx").unwrap();

    cmd.arg("retry")
        .arg("-c")
        .arg("2")
        .arg("--")
        .arg("dummy")
        .assert()
        .failure()
        .stderr(predicate::eq(
            r"cx: command not found 'dummy'
cx: command not found 'dummy'
",
        ));
}

#[test]
fn sleep_one_time() {
    let mut cmd = Command::cargo_bin("cx").unwrap();

    let now = std::time::Instant::now();

    cmd.arg("retry")
        .arg("-c")
        .arg("2")
        .arg("-i")
        .arg("0.5")
        .arg("--")
        .arg("dummy")
        .assert()
        .failure();

    assert!(now.elapsed() >= std::time::Duration::from_secs_f64(0.5))
}
