//! Integration tests for decode command

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

// T021: Integration test for decode from argument
#[test]
fn decode_from_argument_creates_file() {
    let mut cmd = Command::cargo_bin("decodeck").unwrap();
    cmd.arg("decode")
        .arg("SGVsbG8gV29ybGQh")
        .assert()
        .success()
        .stdout(predicate::str::contains("Decoded:"));
}

#[test]
fn decode_from_argument_correct_content() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.txt");

    let mut cmd = Command::cargo_bin("decodeck").unwrap();
    cmd.arg("decode")
        .arg("SGVsbG8gV29ybGQh")
        .arg("--output")
        .arg(output_path.to_str().unwrap())
        .assert()
        .success();

    let content = fs::read_to_string(&output_path).expect("Should read output file");
    assert_eq!(content, "Hello World!");
}

// T022: Integration test for decode from file
#[test]
fn decode_from_file_creates_output() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.b64");
    let output_path = temp_dir.path().join("output.txt");

    fs::write(&input_path, "SGVsbG8gV29ybGQh").expect("Write input file");

    let mut cmd = Command::cargo_bin("decodeck").unwrap();
    cmd.arg("decode")
        .arg("--file")
        .arg(input_path.to_str().unwrap())
        .arg("--output")
        .arg(output_path.to_str().unwrap())
        .assert()
        .success();

    let content = fs::read_to_string(&output_path).expect("Should read output file");
    assert_eq!(content, "Hello World!");
}

#[test]
fn decode_from_nonexistent_file_errors() {
    let mut cmd = Command::cargo_bin("decodeck").unwrap();
    cmd.arg("decode")
        .arg("--file")
        .arg("/nonexistent/path/file.b64")
        .assert()
        .failure();
}

// T023: Integration test for decode from stdin
#[test]
fn decode_from_stdin_creates_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.txt");

    let mut cmd = Command::cargo_bin("decodeck").unwrap();
    cmd.arg("decode")
        .arg("--output")
        .arg(output_path.to_str().unwrap())
        .write_stdin("SGVsbG8gV29ybGQh")
        .assert()
        .success();

    let content = fs::read_to_string(&output_path).expect("Should read output file");
    assert_eq!(content, "Hello World!");
}

// Error cases
#[test]
fn decode_invalid_base64_shows_error() {
    let mut cmd = Command::cargo_bin("decodeck").unwrap();
    cmd.arg("decode").arg("Invalid$Base64!").assert().failure();
}
