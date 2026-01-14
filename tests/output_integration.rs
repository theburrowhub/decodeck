//! Integration tests for output formatting

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

// T041: Integration test for text output format
mod text_output {
    use super::*;

    #[test]
    fn text_output_shows_decoded_path() {
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg("SGVsbG8gV29ybGQh")
            .assert()
            .success()
            .stdout(predicate::str::contains("Decoded:"));
    }

    #[test]
    fn text_output_shows_size() {
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg("SGVsbG8gV29ybGQh")
            .assert()
            .success()
            .stdout(predicate::str::contains("Size:"))
            .stdout(predicate::str::contains("bytes"));
    }

    #[test]
    fn text_output_shows_mime_type() {
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg("SGVsbG8gV29ybGQh")
            .assert()
            .success()
            .stdout(predicate::str::contains("Type:"));
    }

    #[test]
    fn text_output_shows_extension() {
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg("SGVsbG8gV29ybGQh")
            .assert()
            .success()
            .stdout(predicate::str::contains("Extension:"));
    }

    #[test]
    fn text_output_shows_encoding_info() {
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg("SGVsbG8gV29ybGQh")
            .assert()
            .success()
            .stdout(predicate::str::contains("Encoding:"));
    }

    #[test]
    fn text_output_detects_png() {
        // 1x1 red PNG encoded in base64
        let png_b64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg(png_b64)
            .assert()
            .success()
            .stdout(predicate::str::contains("image/png"))
            .stdout(predicate::str::contains(".png"));
    }
}

// T042: Integration test for JSON output format
mod json_output {
    use super::*;

    #[test]
    fn json_flag_produces_json() {
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg("SGVsbG8gV29ybGQh")
            .arg("--json")
            .assert()
            .success()
            .stdout(predicate::str::contains("{"))
            .stdout(predicate::str::contains("}"));
    }

    #[test]
    fn json_output_has_success_field() {
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg("SGVsbG8gV29ybGQh")
            .arg("--json")
            .assert()
            .success()
            .stdout(predicate::str::contains("\"success\": true"));
    }

    #[test]
    fn json_output_has_output_section() {
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg("SGVsbG8gV29ybGQh")
            .arg("--json")
            .assert()
            .success()
            .stdout(predicate::str::contains("\"output\":"))
            .stdout(predicate::str::contains("\"path\":"))
            .stdout(predicate::str::contains("\"size_bytes\":"));
    }

    #[test]
    fn json_output_has_metadata_section() {
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg("SGVsbG8gV29ybGQh")
            .arg("--json")
            .assert()
            .success()
            .stdout(predicate::str::contains("\"metadata\":"))
            .stdout(predicate::str::contains("\"mime_type\":"))
            .stdout(predicate::str::contains("\"extension\":"));
    }

    #[test]
    fn json_output_has_encoding_section() {
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg("SGVsbG8gV29ybGQh")
            .arg("--json")
            .assert()
            .success()
            .stdout(predicate::str::contains("\"encoding\":"))
            .stdout(predicate::str::contains("\"variant\":"));
    }

    #[test]
    fn json_output_is_parseable() {
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        let output = cmd
            .arg("decode")
            .arg("SGVsbG8gV29ybGQh")
            .arg("--json")
            .output()
            .expect("Failed to execute");

        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        let _: serde_json::Value =
            serde_json::from_str(&stdout).expect("Output should be valid JSON");
    }

    #[test]
    fn json_output_detects_png_metadata() {
        let png_b64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";
        let mut cmd = Command::cargo_bin("decodeck").unwrap();
        cmd.arg("decode")
            .arg(png_b64)
            .arg("--json")
            .assert()
            .success()
            .stdout(predicate::str::contains("\"mime_type\": \"image/png\""))
            .stdout(predicate::str::contains("\"category\": \"image\""))
            .stdout(predicate::str::contains("\"is_viewable\": true"));
    }
}
