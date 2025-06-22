pub mod common;

use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

use crate::common::{with_tempdir, CRATE_NAME};

#[test]
fn hack_batch_directory_not_found() {
    let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
    cmd.args([
        "hack",
        "batch",
        "-c",
        "cat",
        "-i",
        "echo hello",
        "-d",
        "/nonexistent/directory",
    ]);
    cmd.assert().success().stderr(predicate::str::contains("is not found"));
}

#[test]
fn hack_batch_invalid_command() {
    with_tempdir(|tempdir| {
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "hack",
            "batch",
            "-c",
            "nonexistent_command_xyz",
            "-i",
            "echo hello",
            "-d",
            tempdir.path().to_str().unwrap(),
            "-t",
            "100",
        ]);
        // Should handle invalid commands gracefully
        cmd.assert().success();
    });
}

#[test]
fn hack_batch_with_output_generator() {
    with_tempdir(|tempdir| {
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "hack",
            "batch",
            "-c",
            "nonexistent_command_xyz",
            "-i",
            "echo input",
            "-o",
            "echo output",
            "-d",
            tempdir.path().to_str().unwrap(),
            "-t",
            "100",
        ]);
        // Test with output generator
        cmd.assert().success();
    });
}

#[test]
fn hack_batch_alias() {
    with_tempdir(|tempdir| {
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "hack",
            "b",
            "-c",
            "nonexistent_command_xyz",
            "-i",
            "echo test",
            "-d",
            tempdir.path().to_str().unwrap(),
            "-t",
            "100",
        ]);
        // Test batch alias
        cmd.assert().success();
    });
}

#[test]
fn hack_batch_short_alias() {
    with_tempdir(|tempdir| {
        let mut cmd = Command::cargo_bin(CRATE_NAME).unwrap();
        cmd.args([
            "h",
            "b",
            "-c",
            "nonexistent_command_xyz",
            "-i",
            "echo test",
            "-d",
            tempdir.path().to_str().unwrap(),
            "-t",
            "100",
        ]);
        // Test short alias
        cmd.assert().success();
    });
}