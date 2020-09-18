use assert_cmd::Command;
use predicates::prelude::*;

#[cfg(not(target_os = "windows"))]
#[test]
fn echo_2_times() {
    let mut cmd = Command::cargo_bin("cx").unwrap();

    cmd.arg("supervise")
        .arg("-c")
        .arg("2")
        .arg("--")
        .arg("echo abc")
        .assert()
        .success()
        .stdout(predicate::eq("abc\nabc\n"));
}

#[cfg(target_os = "windows")]
#[test]
fn echo_2_times() {
    let mut cmd = Command::cargo_bin("cx").unwrap();

    cmd.arg("supervise")
        .arg("-c")
        .arg("2")
        .arg("--")
        .arg("echo abc")
        .assert()
        .success()
        .stdout(predicate::eq("abc\r\nabc\r\n"));
}

#[test]
fn sleep_one_time() {
    let mut cmd = Command::cargo_bin("cx").unwrap();

    let now = std::time::Instant::now();

    cmd.arg("supervise")
        .arg("-c")
        .arg("2")
        .arg("-i")
        .arg("0.5")
        .arg("--")
        .arg("echo abc")
        .assert()
        .success();

    assert!(now.elapsed() >= std::time::Duration::from_secs_f64(0.5))
}
