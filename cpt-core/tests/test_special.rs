pub mod common;

use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

use crate::common::{with_tempdir, write_sync, CRATE_NAME};

fn prepare_testcase(basedir: &std::path::Path, filename: &str, input: &str, output: &str) {
    let input_path = basedir.join(&format!("{}.in", filename));
    let output_path = basedir.join(&format!("{}.out", filename));
    write_sync(&input_path, input, true);
    write_sync(&output_path, output, true);
}

#[test]
fn test_special_directory_not_found() {
    let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
    cmd.args([
        "test",
        "special",
        "-c",
        "cat",
        "-j",
        "echo AC",
        "-d",
        "/nonexistent/directory",
    ]);
    cmd.assert().failure().stderr(predicate::str::contains("is not found"));
}

#[test]
fn test_special_no_cases() {
    with_tempdir(|tempdir| {
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "test",
            "special",
            "-c",
            "cat",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
        ]);
        cmd.assert().failure().stderr(predicate::str::contains("No case found"));
    });
}

#[test]
fn test_special_basic_success() {
    with_tempdir(|tempdir| {
        prepare_testcase(tempdir.path(), "AC_1", "hello", "hello");
        prepare_testcase(tempdir.path(), "AC_2", "world", "world");
        
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "test",
            "special",
            "-c",
            "cat",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
        ]);
        cmd.assert().success();
    });
}

#[test]
fn test_special_with_timelimit() {
    with_tempdir(|tempdir| {
        prepare_testcase(tempdir.path(), "AC_1", "hello", "hello");
        
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "test",
            "special",
            "-c",
            "cat",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
            "-t",
            "5000",
        ]);
        cmd.assert().success();
    });
}

#[test]
fn test_special_command_exec_failed() {
    with_tempdir(|tempdir| {
        prepare_testcase(tempdir.path(), "AC_1", "hello", "hello");
        
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "test",
            "special",
            "-c",
            "/usr/bin/false",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
        ]);
        cmd.assert().failure().stderr(predicate::str::contains("Failed to spawn"));
    });
}

#[test]
fn test_special_alias() {
    with_tempdir(|tempdir| {
        prepare_testcase(tempdir.path(), "AC_1", "hello", "hello");
        
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "test",
            "s",
            "-c",
            "cat",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
        ]);
        cmd.assert().success();
    });
}

#[test]
fn test_special_short_alias() {
    with_tempdir(|tempdir| {
        prepare_testcase(tempdir.path(), "AC_1", "hello", "hello");
        
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "t",
            "s",
            "-c",
            "cat",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
        ]);
        cmd.assert().success();
    });
}

#[test]
fn test_special_input_only_case() {
    with_tempdir(|tempdir| {
        let input_path = tempdir.path().join("AC_input_only.in");
        write_sync(&input_path, "test input", true);
        
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "test",
            "special",
            "-c",
            "cat",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
        ]);
        cmd.assert().success();
    });
}