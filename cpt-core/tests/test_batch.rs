pub mod common;

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use assert_cmd::prelude::*;
use predicates::prelude::*;

use crate::common::{with_tempdir, write_sync, CRATE_NAME};

fn prepare(basedir: &Path, filename: &str, input: &str, expect: Option<&str>) {
    let input_path = basedir.join(&format!("{}.in", filename));
    let output_path = basedir.join(&format!("{}.out", filename));
    write_sync(&input_path, input, true);
    if let Some(expect) = expect {
        write_sync(&output_path, expect, true);
    }
}

#[test]
fn command_exec_failed() {
    with_tempdir(|tempdir| {
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        prepare(tempdir.path(), "AC_1", "abc", Some("abc"));
        prepare(tempdir.path(), "AC_2", "123\n456", Some("123\n456"));
        prepare(tempdir.path(), "AC_input_only", "hoge", None);

        cmd.args([
            "test",
            "batch",
            "-c",
            "noexistent_command",
            "-d",
            tempdir.path().to_str().unwrap(),
        ]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Failed to spawn"));
    });
}

#[test]
fn directory_not_found() {
    let mut dirpath = PathBuf::new();
    with_tempdir(|tempdir| {
        dirpath = tempdir.path().to_path_buf().clone();
    });
    let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
    cmd.args([
        "test",
        "batch",
        "-c",
        "cat",
        "-d",
        dirpath.to_str().unwrap(),
    ]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("is not found"));
}

#[test]
fn testcase_not_found() {
    with_tempdir(|tempdir| {
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "test",
            "batch",
            "-c",
            "cat",
            "-d",
            tempdir.path().to_str().unwrap(),
        ]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("No case found"));
    });
}

#[test]
fn testcase_pass() {
    with_tempdir(|tempdir| {
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        prepare(tempdir.path(), "AC_1", "abc", Some("abc"));
        prepare(tempdir.path(), "AC_2", "123\n456", Some("123\n456"));
        prepare(tempdir.path(), "AC_input_only", "hoge", None);

        cmd.args([
            "test",
            "batch",
            "-c",
            "cat",
            "-d",
            tempdir.path().to_str().unwrap(),
        ]);
        cmd.assert().success();
    });
}
