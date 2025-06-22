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
fn hack_reactive_directory_not_found() {
    let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
    cmd.args([
        "hack",
        "reactive",
        "-c",
        "cat",
        "-i",
        "echo hello",
        "-j",
        "echo AC",
        "-d",
        "/nonexistent/directory",
    ]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("is not found"));
}

#[test]
fn hack_reactive_invalid_command() {
    with_tempdir(|tempdir| {
        prepare_testcase(tempdir.path(), "test1", "input1", "output1");

        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "hack",
            "reactive",
            "-c",
            "nonexistent_command_xyz",
            "-i",
            "echo hello",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
            "-t",
            "100",
        ]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Failed to spawn"));
    });
}

#[test]
fn hack_reactive_with_timelimit() {
    with_tempdir(|tempdir| {
        prepare_testcase(tempdir.path(), "test1", "input1", "output1");

        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "hack",
            "reactive",
            "-c",
            "nonexistent_command_xyz",
            "-i",
            "echo input",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
            "-t",
            "50",
        ]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Failed to spawn"));
    });
}

#[test]
fn hack_reactive_alias() {
    with_tempdir(|tempdir| {
        prepare_testcase(tempdir.path(), "test1", "input1", "output1");

        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "hack",
            "r",
            "-c",
            "nonexistent_command_xyz",
            "-i",
            "echo test",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
            "-t",
            "100",
        ]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Failed to spawn"));
    });
}

#[test]
fn hack_reactive_short_alias() {
    with_tempdir(|tempdir| {
        prepare_testcase(tempdir.path(), "test1", "input1", "output1");

        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "h",
            "r",
            "-c",
            "nonexistent_command_xyz",
            "-i",
            "echo test",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
            "-t",
            "100",
        ]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Failed to spawn"));
    });
}

#[test]
fn hack_reactive_empty_directory() {
    with_tempdir(|tempdir| {
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "hack",
            "reactive",
            "-c",
            "nonexistent_command_xyz",
            "-i",
            "echo test",
            "-j",
            "echo AC",
            "-d",
            tempdir.path().to_str().unwrap(),
            "-t",
            "100",
        ]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Failed to spawn"));
    });
}
